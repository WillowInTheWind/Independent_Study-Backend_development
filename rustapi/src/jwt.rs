use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header};
use axum_extra::extract::cookie::Cookie;
use crate::{AppError, Claims, KEYS, TOKEN_LENGTH_SECONDS};
use crate::handlers::user_manager::UserService;
use crate::state::AppState;
use crate::types::GoogleUser;

pub async fn create_jwt_token(state: AppState, user_id: i64) -> Result<CookieJar, AppError> {
// println!("passed conditional");

    let mut now = Utc::now();
    let expires_in = Duration::try_seconds(TOKEN_LENGTH_SECONDS).unwrap();
    now += expires_in;
    let exp = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id as i32,
        exp
    };
    let jwttoken = encode(
        &Header::default(),
        &claims,
        &KEYS.encoding,
    ).unwrap();

    let mut jar = CookieJar::new().add(Cookie::build(("token", jwttoken)).secure(true).http_only(true).path("/"));
    Ok(jar)
}
