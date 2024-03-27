use std::collections::HashMap;
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl, RequestTokenError, StandardErrorResponse};
use anyhow::{Context, Result};
use async_session::chrono::Utc;
use axum::extract::{Host, Query, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use oauth2::reqwest::http_client;
use oauth2::url::Url;
use uuid::Uuid;
use crate::state::AppState;
use crate::types;
use crate::types::{GenericUser, OauthError};
use reqwest::get;
use std::env;
use async_session::log::error;
use async_session::Session;
use axum::Extension;
use axum::http::StatusCode;
use oauth2::basic::BasicErrorResponseType;
use serde::Deserialize;
use crate::handlers::user_manager::UserService;

pub(crate) fn oauth_client() -> Result<BasicClient, crate::types::OauthError> {
    let client_id = ClientId::new(env::var("CLIENT_ID")?);
    let client_secret = ClientSecret::new(env::var("CLIENT_SECRET")?);
    let redirect_url = "http://127.0.0.1:8080/auth/authorized".to_string();

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())
        .map_err(|_| "OAuth: invalid authorization endpoint URL")?;

    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/token".to_string())
        .map_err(|_| "OAuth: invalid token endpoint URL")?;




    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
        .set_redirect_uri(RedirectUrl::new(redirect_url).map_err(|_| "OAuth: invalid redirect URL")?)
        .set_revocation_uri(
            RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
                .map_err(|_| "OAuth: invalid revocation endpoint URL")?,
        );
    Ok(client)
}


pub(crate) async fn login(Extension(user_data): Extension<Option<GenericUser>>, State(state): State<AppState>, Query(mut params): Query<HashMap<String, String>>, ) -> Result<Redirect, authError > {
    // if user_data.is_some() {
    //     // check if already authenticated
    //     return Ok(Redirect::to("/"));
    // // }
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _csrf_token) = state.oauth_client.clone()
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))        .set_pkce_challenge(pkce_code_challenge)
        .url();


     sqlx::query(
        "INSERT INTO oauth2_state_storage (csrf_state, pkce_code_verifier, return_url) VALUES (?, ?, ?);",
    )
        .bind(_csrf_token.secret())
        .bind(pkce_code_verifier.secret())
        .bind("http://127.0.0.1:8080/auth/authorized")
        .execute(&state.dbreference)
        .await?;
    Ok(Redirect::to(&auth_url.to_string()))
}

pub async fn oauth_return(
    Query(mut params): Query<HashMap<String, String>>,
    State(Appstate): State<AppState>,
    Host(hostname): Host,
) -> Result<impl IntoResponse, authError> {

    let state = CsrfToken::new(params.remove("state").ok_or("OAuth: without state")?);
    let code = AuthorizationCode::new(params.remove("code").ok_or("OAuth: without code")?);

    let query: (String, String) = sqlx::query_as(
        r#"DELETE FROM oauth2_state_storage WHERE csrf_state = ? RETURNING pkce_code_verifier,return_url"#,
    )
        .bind(state.secret())
        .fetch_one(&Appstate.dbreference)
        .await?;

    // Get an auth token
    println!("{:?}", code);

    let pkce_code_verifier = query.0;
    let return_url = query.1;
    let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier);

    // Exchange the code with a token.
    let client = Appstate.oauth_client.clone();
    let token_response = client.exchange_code(code).request_async(async_http_client).await.map_err(|e| authError::new("Internal server error"))?;
    let access_token = token_response.access_token().secret();
    let url =
        "https://www.googleapis.com/oauth2/v2/userinfo?oauth_token=".to_owned() + access_token;
    let body = reqwest::get(url)
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
        return Err(authError::new("OAuth: email address is not verified".to_owned())
            .with_user_message("Your email address is not verified. Please verify your email address with Google and try again.".to_owned()));
    }
    let query: Result<(i64,), _> = sqlx::query_as(r#"SELECT id FROM users WHERE email=?"#)
        .bind(email.as_str())
        .fetch_one(&Appstate.dbreference)
        .await;
    let user_id = if let Ok(query) = query {
        query.0
    } else {
        let query: (i64,) = sqlx::query_as("INSERT INTO users (email) VALUES (?) RETURNING id")
            .bind(email)
            .fetch_one(&Appstate.dbreference)
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
        .bind(now + 60 * 60 * 24)
        .execute(&Appstate.dbreference)
        .await?;

    Ok((headers, Redirect::to(return_url.as_str())))
    }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug)]
pub(crate) struct ReturnError(anyhow::Error);

impl IntoResponse for ReturnError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}
impl<E> From<E> for ReturnError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub struct authError {
    code: StatusCode,
    message: String,
    user_message: String,
}
impl authError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            user_message: "".to_owned(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn with_user_message(self, user_message: impl Into<String>) -> Self {
        Self {
            user_message: user_message.into(),
            ..self
        }
    }
    // pub fn with_code(self, code: StatusCode) -> Self {
    //     Self {
    //         code,
    //         ..self
    //     }
    // }
}

impl IntoResponse for authError {
    fn into_response(self) -> axum::response::Response {
        println!("AppError: {}", self.message);
        (
            self.code,
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title>Oops!</title>
                </head>
                <body>
                    <h1>Oops!</h1>
                    <p>Sorry, but something went wrong.</p>
                    <p>{}</p>
                </body>
                </html>
                "#,
                self.user_message
            )),
        )
            .into_response()
    }
}




impl From<sqlx::Error> for authError {
    fn from(err: sqlx::Error) -> Self {
        authError::new(format!("Database query error: {:#}", err))
    }
}

impl From<String> for authError {
    fn from(err: String) -> Self {
        authError::new(err)
    }
}

impl From<&str> for authError {
    fn from(err: &str) -> Self {
        authError::new(err)
    }
}

// impl From<RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>> for authError {
//     fn from(err: RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>) -> Self {
//         authError::new(err)
//     }
// }