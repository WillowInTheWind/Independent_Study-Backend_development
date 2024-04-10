use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use axum::extract::{Query, State};
use oauth2::basic::BasicClient;
use axum::response::{IntoResponse, Redirect, Response};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use oauth2::reqwest::async_http_client;
use http::StatusCode;
use anyhow::Context;
use axum::Json;
use chrono::{Duration, NaiveDate, Utc};
use serde_json::json;
use sqlx::encode::IsNull::No;
use crate::{AppError, jwt};
use crate::defaultroutes::user_manager::UserService;
use crate::state::AppState;
use crate::types::{CalendarEvent, GoogleUser, MorningExercise};

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static TOKEN_LENGTH_SECONDS: i64 = 12*60*60;

pub(crate) async fn login_authorized(
    State(state): State<AppState>,
    State(oauth_client): State<BasicClient>,
    Query(query): Query<AuthRequest>) -> Result<Response, AppError> {
    println!("->> reached authorization page");

    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await?;
    // Fetch user data from Google


    let mut user_data/* Type */ = state.reqwestClient
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<GoogleUser>()
        .await
        .context("failed to deserialize response as JSON")?
        ;

    println!("->> user data found ");
    // let mx = MorningExercise::new_with_date(
    //     1, user_data.clone(), NaiveDate::from_ymd_opt(2024,6,6).unwrap(), "WOW".to_string(), "WOW".to_string(), None
    // );
    //
    //
    // let date = &mx.date.and_hms_opt(10,50,0);
    // let enddate = &mx.date.and_hms_opt(11,30,0);

    // let event = CalendarEvent::new(mx.clone().title, date.unwrap(), enddate.unwrap());
    // let calendar/* Type */ = state.reqwestClient
    //     .post("https://www.googleapis.com/calendar/v3/calendars/wayland.chase@gmail.com/events")
    //     .bearer_auth(token.access_token().secret())
    //     .json(&event)
    //     .send()
    //     .await
    //     .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)
    //     ;

    // println!("{:?}",calendar);

    let user_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM GoogleUsers WHERE name = $1)")
            .bind(&user_data.name)
            .fetch_one(&state.dbreference)
            .await
            .context("failed in finding if user exists").unwrap();

    if user_exists {
        let user = state.dbreference.get_user_by_name(&user_data.name).await?;
        state.dbreference.reset_user_token(token.access_token().secret().to_string(), user.id.unwrap()).await;
        let jar = jwt::create_jwt_token(user.id.unwrap()).await?;
        return Ok((jar, Redirect::to("/")).into_response())
    }

    let user = GoogleUser {
        id: Some(1),
        sub: user_data.sub,
        picture: user_data.picture,
        email: user_data.email,
        name: user_data.name,
        token: Some(token.access_token().secret().to_string()),
        phone_number: None
    };

    let user_id = state.dbreference.create_user(user).await?;
    let jar = jwt::create_jwt_token(user_id).await?;
    Ok((jar, Redirect::to("/")).into_response())
}
pub(crate) async fn logout(
) -> Result<Response, StatusCode> {
    println!("->> user logged out");

    let cookies = jwt::remove_jwt_token().await.map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((cookies, Redirect::to("/api")).into_response())
}

pub(crate) async fn login(State(client): State<BasicClient>) -> Response {
    println!("->> user logged in or signed up");

    // TODO: this example currently doesn't validate the CSRF token during login attempts. That
    // makes it vulnerable to cross-site request forgery. If you copy code from this example make
    // sure to add a check for the CSRF token.
    // Issue for adding check to this example https://github.com/tokio-rs/axum/issues/2511

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/calendar".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    let url = auth_url.to_string();
    let authurl = Json(url);
    authurl.into_response()
    // Redirect::to(&auth_url.to_string()).into_response()
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct event {
    summary: String,
    // start: eventtime,
    // end: eventtime
}
#[derive(Serialize, Deserialize, Debug)]

struct eventtime {
    date: chrono::NaiveDate,
    dateTime: chrono::DateTime<Utc>,
    timeZone: String
}