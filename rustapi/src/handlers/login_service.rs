use std::collections::HashMap;
use std::env;
use oauth2::{basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl};
use anyhow::{Context, Result};
use async_session::chrono::Utc;
use axum::extract::{Host, Query, State};
use axum::response::{IntoResponse, Redirect};
use oauth2::reqwest::http_client;
use oauth2::url::Url;
use uuid::Uuid;
use crate::state::AppState;
use crate::types;
use crate::types::OauthError;
use reqwest::get;
pub(crate) fn oauth_client() -> Result<BasicClient, crate::types::OauthError> {
    // Environment variables (* = required):
    // *"CLIENT_ID"     "REPLACE_ME";
    // *"CLIENT_SECRET" "REPLACE_ME";
    //  "REDIRECT_URL"  "http://127.0.0.1:3000/auth/authorized";
    //  "AUTH_URL"      "https://discord.com/api/oauth2/authorize?response_type=code";
    //  "TOKEN_URL"     "https://discord.com/api/oauth2/token";

    let client_id = env::var("CLIENT_ID").context("Missing CLIENT_ID!").map_err( types::AppError);
    let client_secret = env::var("CLIENT_SECRET").context("Missing CLIENT_SECRET!")    .map_err( types::AppError);
    let redirect_url = env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000/auth/authorized".to_string());

    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| {
        "https://discord.com/api/oauth2/authorize?response_type=code".to_string()
    });

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

    Ok(BasicClient::new(ClientId::new(client_id.unwrap()),
                        Some(ClientSecret::new(client_secret.unwrap())),
                        AuthUrl::new(auth_url).context("failed to create new authorization server URL").unwrap(),
                        Some(TokenUrl::new(token_url).context("failed to create new token endpoint URL").unwrap(),
    )).set_redirect_uri(
            RedirectUrl::new(redirect_url).context("failed to create new redirection URL").map_err(types::AppError).unwrap(),
    ))
}


async fn discord_auth(State(client): State<AppState>) -> std::result::Result<Redirect, OauthError> {
    // if user_data.is_some() {
    //     // check if already authenticated
    //     return Ok(Redirect::to("/"));
    // }
    // let return_url = params
    //     .remove("return_url")
    //     .unwrap_or_else(|| "/".to_string());
    // TODO: check if return_url is valid

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token): (Url, CsrfToken) = client.oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();


    /* sqlx::query(
        "INSERT INTO oauth2_state_storage (csrf_state, pkce_code_verifier, return_url) VALUES (?, ?, ?);",
    )
        .bind(csrf_state.secret())
        .bind(pkce_code_verifier.secret())
        .bind(return_url)
        .execute(&db_pool)
        .await?; */
    Ok( Redirect::to(auth_url.as_ref()))
    // Redirect to Google's oauth service
}

pub async fn oauth_return(
    Query(mut params): Query<HashMap<String, String>>,
    State(state): State<AppState>,
    Host(hostname): Host,
) -> Result<impl IntoResponse, OauthError> {
    let csrf_token = CsrfToken::new(params.remove("state").ok_or("OAuth: without state")?);
    let code = AuthorizationCode::new(params.remove("code").ok_or("OAuth: without code")?);

    let query: (String, String) = sqlx::query_as(
        r#"DELETE FROM oauth2_state_storage WHERE csrf_state = ? RETURNING pkce_code_verifier,return_url"#,
    )
        .bind(csrf_token.secret())
        .fetch_one(&state.dbreference)
        .await?;


    let pkce_code_verifier = query.0;
    let return_url = query.1;
    let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier);

    // Exchange the code with a token.
    let client = state.oauth_client.clone();
    let token_response = tokio::task::spawn_blocking(move || {
        client
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request(http_client)
    })
        .await
        .map_err(|_| "OAuth: exchange_code failure")?
        .map_err(|_| "OAuth: tokio spawn blocking failure")?;
    let access_token = token_response.access_token().secret();

    // Get user info from Google
    let url =
        "https://www.googleapis.com/oauth2/v2/userinfo?oauth_token=".to_owned() + access_token;
    let body = get(url)
        .await
        .map_err(|_| "OAuth: reqwest failed to query userinfo")?
        .text()
        .await
        .map_err(|_| "OAuth: reqwest received invalid userinfo")?;
    let mut body: serde_json::Value =
        serde_json::from_str(body.as_str()).map_err(|_| "OAuth: Serde failed to parse userinfo")?;
    let email = body["email"]
        .take()
        .as_str()
        .ok_or("OAuth: Serde failed to parse email address")?
        .to_owned();
    let verified_email = body["verified_email"]
        .take()
        .as_bool()
        .ok_or("OAuth: Serde failed to parse verified_email")?;
    if !verified_email {
        return Err(OauthError::new("OAuth: email address is not verified".to_owned())
            .with_user_message("Your email address is not verified. Please verify your email address with Google and try again.".to_owned()));
    }

    // Check if user exists in database
    // If not, create a new user
    let query: Result<(i64,), _> =
        sqlx::query_as(r#"SELECT id FROM user WHERE user_email = ?"#)
            .bind(email.as_str())
        .fetch_one(&state.dbreference)
        .await;
    let user_id = if let Ok(query) = query {
        query.0
    } else {
        let query: (i64,) = sqlx::query_as("INSERT INTO users (email) VALUES (?) RETURNING id")
            .bind(email)
            .fetch_one(&state.dbreference)
            .await?;
        query.0
    };

    // Create a session for the user
    let session_token_p1 = Uuid::new_v4().to_string();
    let session_token_p2 = Uuid::new_v4().to_string();
    let session_token = [session_token_p1.as_str(), "_", session_token_p2.as_str()].concat();
    let headers = axum::response::AppendHeaders([(
        axum::http::header::SET_COOKIE,
        "session_token=".to_owned()
            + &*session_token
            + "; path=/; httponly; secure; samesite=strict",
    )]);
    let now = Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO user_sessions
        (session_token_p1, session_token_p2, user_id, created_at, expires_at)
        VALUES (?, ?, ?, ?, ?);",
    )
        .bind(session_token_p1)
        .bind(session_token_p2)
        .bind(user_id)
        .bind(now)
        .bind(now + 60 * 60 * 24 * 7)
        .execute(&state.dbreference)
        .await?;

    Ok((headers, Redirect::to(return_url.as_str())))
}
