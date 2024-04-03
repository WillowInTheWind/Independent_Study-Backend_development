use crate::{AuthRedirect, Claims};
use crate::{AppState};
use axum::extract::{FromRef, FromRequestParts, State};
use axum::{Extension, Json, RequestPartsExt};
use axum::response::IntoResponse;
use axum_macros::debug_handler;
use crate::handlers::user_manager::UserService;
use crate::types;
use crate::types::{GenericUser, GoogleUser};
use async_session::{MemoryStore, SessionStore};
use async_trait::async_trait;

use axum_extra::typed_header::TypedHeaderRejectionReason;
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;
use http::{header, StatusCode};
use http::request::Parts;
use jsonwebtoken::decode;

static COOKIE_NAME: &str = "SESSION";

pub mod user_manager;
pub(crate) mod mx_service;
// static COOKIE_NAME: &str = "SESSION";
pub(crate) async fn error_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}
#[debug_handler]
pub(crate) async fn root(Extension(user): Extension<GoogleUser>,
                         State(state): State<AppState>,
) -> impl IntoResponse {
   format!(
            "Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
            user.name
        )

}

#[debug_handler]
pub async fn users(
    State(state): State<AppState>,
) -> Json<Vec<GoogleUser>> {
   Json(state.dbreference.get_users().await.map_err(types::internal_error).unwrap())
}


// #[async_trait]
// impl<S> FromRequestParts<S> for GoogleUser
//     where
//     MemoryStore: FromRef<S>,
//     S: Send + Sync
// {
//     // If anything goes wrong or no session is found, redirect to the auth page
//     type Rejection = AuthRedirect;
//
//     async fn from_request_parts(parts: &mut Parts,
//     state: &S)
//         -> Result<Self, Self::Rejection> {
//         let store = MemoryStore::from_ref(state);
//
//         let cookies = parts
//             .extract::<TypedHeader<headers::Cookie>>()
//             .await
//             .map_err(|e| match *e.name() {
//                 header::COOKIE => match e.reason() {
//                     TypedHeaderRejectionReason::Missing => AuthRedirect,
//                     _ => panic!("unexpected error getting Cookie header(s): {e}"),
//                 },
//                 _ => panic!("unexpected error getting cookies: {e}"),
//             })?;
//         let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;
//
//         let session = store
//             .load_session(session_cookie.to_string())
//             .await
//             .unwrap()
//             .ok_or(AuthRedirect)?;
//
//         let user = session.get::<GoogleUser>("user").ok_or(AuthRedirect)?;
//
//         Ok(user)
//     }
// }
//
