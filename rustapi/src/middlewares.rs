use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use http::{header, HeaderValue, StatusCode};
use jsonwebtoken::{decode, Validation};
use serde::Serialize;
use crate::defaultroutes::user_manager::UserService;
use crate::state::AppState;
use axum_extra::extract::cookie::CookieJar;
use crate::loginroutes::{Claims, KEYS};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn auth(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let hi = req.headers().get("authorization");
    let token = hi.ok_or_else(|| {
        StatusCode::UNAUTHORIZED
    })?;
println!("token found");
    let token = token.to_str().unwrap().replace("token=", "");

    let claims = decode::<Claims>(
        &token,
        &KEYS.decoding,
        &Validation::default(),
    )
        .map_err(|e| {
            StatusCode::UNAUTHORIZED
        })?
        .claims;

    let user_id = claims.sub;
    let user = state.dbreference.get_user_by_id(user_id).await.unwrap();
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}