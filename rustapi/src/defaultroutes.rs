use crate::{AppState};
use axum::extract::{ State};
use axum::{Extension};
use axum::response::IntoResponse;
use axum_macros::debug_handler;
use crate::types::{ GoogleUser};
use http::{ StatusCode};
pub mod user_manager;
pub(crate) mod mx_service;
pub(crate) mod calendarservice;
mod emailservice;

pub(crate) async fn error_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}
#[debug_handler]
pub(crate) async fn root(Extension(user): Extension<GoogleUser>,
                         State(state): State<AppState>,
) -> impl IntoResponse {
    let data = state.reqwestClient
        .get("https://www.googleapis.com/calendar/v3/calendars/calendarId/events/eventId");
   format!(
            "Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
            user.name
        ).into_response()
}

