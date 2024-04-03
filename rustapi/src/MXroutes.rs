use axum::extract::State;
use axum::{Extension, Json};
use axum::response::Response;
use axum_macros::debug_handler;
use crate::state::AppState;
use crate::defaultroutes::mx_service::MxService;
use crate::types::{GoogleUser, MorningExercise};

#[debug_handler]
pub async fn get_all_mxs(
    State(app_state): State<AppState>
) -> Json<Vec<MorningExercise>> {
    Json(app_state.dbreference.get_mxs().await.unwrap())
}
pub async fn get_users_mxs(
    Extension(user): Extension<GoogleUser>,
    State(state): State<AppState>
) -> Json<Vec<MorningExercise>> {
    Json(state.dbreference.get_mxs_by_owner(user.id.unwrap()).await.unwrap())
}
pub async fn post_mx() -> Response {
    todo!()
}
pub async fn delete_mx() -> Response {
    todo!()
}