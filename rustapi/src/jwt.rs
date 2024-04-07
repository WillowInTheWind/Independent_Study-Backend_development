use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header};
use axum_extra::extract::cookie::{Cookie, SameSite};
use http::header::{AUTHORIZATION, SET_COOKIE};
use crate::AppError;
use crate::loginroutes::{Claims, KEYS, TOKEN_LENGTH_SECONDS};

use http::HeaderMap;

pub async fn create_jwt_token(user_id: i64) -> Result<CookieJar, AppError> {
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


    let jar = CookieJar::new()
        .add(Cookie::build(("token", jwttoken))
            .secure(true)
            .same_site(SameSite::Lax)
            .path("/"));
    Ok(jar)
    // Ok(jar)
}
pub async fn remove_jwt_token () -> Result<CookieJar, AppError> {
    let jar = CookieJar::new().add(Cookie::build(("token", "")).secure(true).path("/"));
    Ok(jar)
}