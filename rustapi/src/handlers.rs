use anyhow::{anyhow, Context};
use axum::extract::State;
// use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Redirect};
use crate::state::AppState;
use axum_macros::debug_handler;
use oauth2::{CsrfToken, Scope};
use crate::handlers::user_manager::UserService;
use crate::types;
use crate::types::{AppError, GenericUser};
use async_session::{MemoryStore, Session, SessionStore};
use axum::http::header::SET_COOKIE;
use axum::http::HeaderMap;
use serde::Deserialize;

pub mod user_manager;
pub(crate) mod login_service;
pub(crate) mod mx_service;
// static COOKIE_NAME: &str = "SESSION";
#[debug_handler]
pub async fn root( ) -> &'static str {
    "This is the main route of the server"
}
#[debug_handler]
pub async fn users(
    State(state): State<AppState>,
) -> Json<Vec<GenericUser>> {
   Json(state.dbreference.get_users().await.map_err(types::internal_error).unwrap())
}

async fn discord_auth(State(client): State<AppState>) -> impl IntoResponse {
    // Issue for adding check to this example https://github.com/tokio-rs/axum/issues/2511
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();
    let mut session = Session::new();
    session
        .insert("csrf_token", &csrf_token)
        .context("failed in inserting CSRF token into session")?;

    // Store the session in MemoryStore and retrieve the session cookie
    let cookie = store
        .store_session(session)
        .await
        .context("failed to store CSRF token session")?
        .context("unexpected error retrieving CSRF cookie value")?;

    // Attach the session cookie to the response header
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to(auth_url.as_ref())))
    // Redirect to Google's oauth service
}
// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// struct AuthRequest {
//     code: String,
//     state: String,
// }
// async fn csrf_token_validation_workflow(
//     auth_request: &AuthRequest,
//     cookies: &headers::Cookie,
//     State(state): State<AppState>,
// ) -> Result<(), AppError> {
//     // Extract the cookie from the request
//     let cookie = cookies
//         .get(COOKIE_NAME)
//         .context("unexpected error getting cookie name")?
//         .to_string();
//
//     // Load the session
//     let session = match state.dbreference
//         .load_session(cookie)
//         .await
//         .context("failed to load session")?
//     {
//         Some(session) => session,
//         None => return Err(anyhow!("Session not found").into()),
//     };
//
//     // Extract the CSRF token from the session
//     let stored_csrf_token = session
//         .get::<CsrfToken>("csrf_token")
//         .context("CSRF token not found in session")?
//         .to_owned();
//
//     // Cleanup the CSRF token session
//     State(state)
//         .destroy_session(session)
//         .await
//         .context("Failed to destroy old session")?;
//
//     // Validate CSRF token is the same as the one in the auth request
//     if *stored_csrf_token.secret() != auth_request.state {
//         return Err(anyhow!("CSRF token mismatch").into());
//     }
//
//     Ok(())
// }