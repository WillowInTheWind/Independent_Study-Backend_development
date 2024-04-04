use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::Response;
use axum_macros::debug_handler;
use http::StatusCode;
use crate::defaultroutes::user_manager::UserService;
use crate::state::AppState;
use crate::types;
use crate::types::GoogleUser;


pub async fn delete_user() -> Response {
    todo!()
}
pub async fn get_user_by_id(Path(params): Path<i32>,State(state): State<AppState>,) -> Json<GoogleUser> {
    Json(state.dbreference.get_user_by_id(params).await.unwrap())
}

pub async fn get_user_by(Path(params): Path<&str>,State(state): State<AppState>,) -> Result<Json<GoogleUser>, StatusCode> {
    match params {
        "email" => {
            Ok(Json(state.dbreference.get_user_by_email(params).await.unwrap()))
        }
        "sub" => {
            Ok(Json(state.dbreference.get_user_by_sub(params).await.unwrap()))        }
        "name" => {
            Ok(Json(state.dbreference.get_user_by_name(params).await.unwrap()) )       }
        _ => {Err(StatusCode::NOT_FOUND)}
    }
}

struct GetParams {

}
#[debug_handler]
pub async fn get_all_users(
    State(state): State<AppState>,
) -> Json<Vec<GoogleUser>> {
    Json(state.dbreference.get_users().await.map_err(types::internal_error).unwrap())
}
