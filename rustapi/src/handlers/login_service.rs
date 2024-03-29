// //! Example OAuth (Discord) implementation.
// //!
// //! 1) Create a new application at <https://discord.com/developers/applications>
// //! 2) Visit the OAuth2 tab to get your CLIENT_ID and CLIENT_SECRET
// //! 3) Add a new redirect URI (for this example: `http://127.0.0.1:3000/auth/authorized`)
// //! 4) Run with the following (replacing values appropriately):
// //! ```not_rust
// //! CLIENT_ID=REPLACE_ME CLIENT_SECRET=REPLACE_ME cargo run -p example-oauth
// //! ```
//
// use anyhow::{Context, Result};
// use async_session::{MemoryStore, Session, SessionStore};
// use axum::{
//     async_trait,
//     extract::{FromRef, FromRequestParts, Query, State},
//     http::{header::SET_COOKIE, HeaderMap},
//     response::{IntoResponse, Redirect, Response},
//     routing::get,
//     RequestPartsExt, Router,
// };
// use axum_extra::{headers, typed_header::TypedHeaderRejectionReason, TypedHeader};
// use http::{header, request::Parts, StatusCode};
// use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl, RevocationUrl};
// use serde::{Deserialize, Serialize};
// use std::env;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use crate::AppState;
// use crate::types::GenericUser;
//
// static COOKIE_NAME: &str = "SESSION";
//
// pub(crate) fn oauth_client() -> Result<BasicClient, crate::types::OauthError> {
//     let client_id = ClientId::new(env::var("CLIENT_ID")?);
//     let client_secret = ClientSecret::new(env::var("CLIENT_SECRET")?);
//     let redirect_url = "http://127.0.0.1:8080/auth/authorized".to_string();
//
//     let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())
//         .map_err(|_| "OAuth: invalid authorization endpoint URL")?;
//
//     let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/token".to_string())
//         .map_err(|_| "OAuth: invalid token endpoint URL")?;
//
//
//
//
//     let client = BasicClient::new(
//         client_id,
//         Some(client_secret),
//         auth_url,
//         Some(token_url),
//     )
//         .set_redirect_uri(RedirectUrl::new(redirect_url).map_err(|_| "OAuth: invalid redirect URL")?);
//     Ok(client)
// }
//
//
// pub(crate) async fn login(State(client): State<BasicClient>) -> impl IntoResponse {
//     // TODO: this example currently doesn't validate the CSRF token during login attempts. That
//     // makes it vulnerable to cross-site request forgery. If you copy code from this example make
//     // sure to add a check for the CSRF token.
//     //
//     // Issue for adding check to this example https://github.com/tokio-rs/axum/issues/2511
//     let (auth_url, _csrf_token) = client
//     .authorize_url(CsrfToken::new_random)
//     .add_scope(Scope::new("profile".to_string()))
//     .add_scope(Scope::new("email".to_string()))
//     .url();
//
//     Redirect::to(&auth_url.to_string())
// }
//
// pub(crate) async fn login_authorized(
//     State(state): State<AppState>,
//     State(oauth_client): State<BasicClient>,
//     Query(query): Query<AuthRequest>,
// ) -> Result<impl IntoResponse, AppError> {
//     let token = oauth_client
//         .exchange_code(AuthorizationCode::new(query.code))
//         .request_async(async_http_client)
//         .await?;
//     // Fetch user data from discord
//     let client = reqwest::Client::new();
//
//
//     Ok(Redirect::to("/"))
// }
//
//
// struct AuthRedirect;
//
// impl IntoResponse for AuthRedirect {
//     fn into_response(self) -> Response {
//         Redirect::temporary("/auth/discord").into_response()
//     }
// }
// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub(crate) struct AuthRequest {
//     code: String,
//     state: String,
// }
//
//
// // Use anyhow, define error and enable '?'
// // For a simplified example of using anyhow in axum check /examples/anyhow-error-response
// #[derive(Debug)]
// pub(crate) struct AppError(anyhow::Error);
//
// // Tell axum how to convert `AppError` into a response.
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         tracing::error!("Application error: {:#}", self.0);
//
//         (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
//     }
// }
//
// // This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// // `Result<_, AppError>`. That way you don't need to do that manually.
// impl<E> From<E> for AppError
//     where
//         E: Into<anyhow::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }