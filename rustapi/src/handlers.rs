use crate::AppState;
use anyhow::{anyhow, Context};
use axum::extract::State;
// use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Redirect};
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
