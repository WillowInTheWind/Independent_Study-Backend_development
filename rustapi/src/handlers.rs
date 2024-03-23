use axum::extract::State;
use axum::Json;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use axum_macros::debug_handler;
use sqlx::FromRow;
use crate::handlers::user_manager::UserService;

pub mod user_manager;
#[derive(FromRow, Debug, Deserialize, Serialize)]
pub(crate) struct GenericUser {
    id: Option<i16>,
    user_name: String,
    //if I end up implementing other way to login besides google Oauth I can change the user type
    user_identifier: i32,
}

#[debug_handler]
pub async fn root( ) -> &'static str {
    "This is the main route of the server"
}
// #[debug_handler]
pub async fn login(
    State(state): State<AppState>,
) -> Json<GenericUser> {

   Json(
       GenericUser {
            id: Some(0),
            user_name: "daddy".to_string(),
            user_identifier: 12465
       }
   )
}
