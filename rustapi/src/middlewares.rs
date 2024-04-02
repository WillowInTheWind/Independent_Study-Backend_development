use axum::body::Body;
use axum::extract::{Request, State};
use axum::Json;
use axum::middleware::Next;
use axum::response::IntoResponse;
use http::{header, StatusCode};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use crate::{Claims, KEYS};
use crate::handlers::user_manager::UserService;
use crate::state::AppState;
use crate::types::GoogleUser;
use axum_extra::extract::cookie::CookieJar;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(|| {
        StatusCode::UNAUTHORIZED
    })?;

    let claims = decode::<Claims>(
        &token,
        &KEYS.decoding,
        &Validation::default(),
    )
        .map_err(|_| {
            StatusCode::UNAUTHORIZED
        })?
        .claims;

    let user_id = claims.sub;

    let user = state.dbreference.get_user_by_id(user_id).await.unwrap();

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}