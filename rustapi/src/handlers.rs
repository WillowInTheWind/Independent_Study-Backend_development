use anyhow::{anyhow, Context};
use axum::extract::State;
// use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Redirect};
use crate::state::AppState;
use axum_macros::debug_handler;
use crate::handlers::user_manager::UserService;
use crate::types;
use crate::types::{OauthError, GenericUser};

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
