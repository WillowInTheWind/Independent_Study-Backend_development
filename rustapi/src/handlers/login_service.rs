use std::collections::HashMap;
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl};
use anyhow::{Context, Result};
use async_session::chrono::Utc;
use axum::extract::{Host, Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use oauth2::reqwest::http_client;
use oauth2::url::Url;
use uuid::Uuid;
use crate::state::AppState;
use crate::types;
use crate::types::{GenericUser, OauthError};
use reqwest::get;
use std::env;
use async_session::Session;
use axum::Extension;
use axum::http::StatusCode;
use serde::Deserialize;
use crate::handlers::user_manager::UserService;

pub(crate) fn oauth_client() -> Result<BasicClient, crate::types::OauthError> {
    let client_id = ClientId::new(env::var("CLIENT_ID")?);
    let client_secret = ClientSecret::new(env::var("CLIENT_SECRET")?);
    let redirect_url = env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080/auth/authorized".to_string());

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
    .set_redirect_uri(
            RedirectUrl::new(redirect_url).context("failed to create new redirection URL").map_err(types::AppError).unwrap(),
    )
        .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .map_err(|_| "OAuth: invalid revocation endpoint URL")?,
    );
    Ok(client)
}


pub(crate) async fn login(Extension(user_data): Extension<Option<GenericUser>>, State(state): State<AppState>, Query(mut params): Query<HashMap<String, String>>, ) -> Redirect {
    // if user_data.is_some() {
    //     // check if already authenticated
    //     return Ok(Redirect::to("/"));
    // // }
    let (auth_url, _csrf_token) = state.oauth_client.clone()
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

//     // Redirect to Google's oauth service
// // println!("{:?}", client);
//      sqlx::query(
//         "INSERT INTO oauth2_state_storage (csrf_state, pkce_code_verifier, return_url) VALUES (?, ?, ?);",
//     )
//         .bind(_csrf_token.secret())
//         .bind(pkce_code_verifier.secret())
//         .bind(return_url)
//         .execute(&state.dbreference)
//         .await?;
    Redirect::to(&auth_url.to_string())
    // Redirect to Google's oauth service
}

pub async fn oauth_return(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
    Host(hostname): Host,
) -> Result<impl IntoResponse, ReturnError> {
    // Get an auth token
    let token = state.oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request request to authorization server")?;
        // Fetch user data from discord
    let client = reqwest::Client::new();
    let user_data: GenericUser = client
            // https://discord.com/developers/docs/resources/user#get-current-user
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(token.access_token().secret())
            .send()
            .await
            .context("failed in sending request to target Url")?
            .json::<GenericUser>()
            .await
            .context("failed to deserialize response as JSON")?;

        // Create a new session filled with user data
        let mut session = Session::new();
        session
            .insert("user", &user_data)
            .context("failed in inserting serialized value into session")?;

        // Store session and get corresponding cookie
        //TODO: store session in DB


        // Build the cookie
        // Set cookie


        Ok( Redirect::to("/"))
    }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug)]
pub(crate) struct ReturnError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for ReturnError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for ReturnError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}