use axum::extract::State;
use axum::Json;
use axum::response::Response;
use axum_macros::debug_handler;
use crate::defaultroutes::user_manager::UserService;
use crate::state::AppState;
use crate::types;
use crate::types::GoogleUser;

pub async fn create_user() -> Response {
    todo!()
}

pub async fn delete_user() -> Response {
    todo!()
}
pub async fn get_user() -> Response {
    todo!()
}

#[debug_handler]
pub async fn get_all_users(
    State(state): State<AppState>,
) -> Json<Vec<GoogleUser>> {
    Json(state.dbreference.get_users().await.map_err(types::internal_error).unwrap())
}
