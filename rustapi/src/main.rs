mod handlers;
mod state;
mod mx_date_algorithm;
mod types;
mod config;
mod middlewares;
mod JWT;

use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use std::string::String;
use axum::{Json, middleware, Router, routing::get};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use dotenv::dotenv;
use anyhow::Context;
use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    RequestPartsExt,
    response::IntoResponse,
};
use axum::extract::{Query, State};
use axum::response::{Redirect, Response};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Duration, TimeDelta, Utc};
use http::header::SET_COOKIE;
use http::{header, HeaderMap, StatusCode};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::handlers::user_manager::UserService;
use crate::middlewares::auth;
use crate::state::AppState;
use crate::types::{GenericUser, GoogleUser};
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
static COOKIE_NAME: &str = "SESSION";
static TOKEN_LENGTH_SECONDS: i64 = 12*60*60;
// #[debug_handler]
struct EnvironmentVariables {
    address: String,
    port: String,
}
#[tokio::main]
async fn main(){
    //Init enviorment variables and tracing
        dotenv().ok();
        tracing_subscriber::fmt::init();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must set");
        let environment_variables = config::initialize_environment_variable().await;
    //Init App State
        let store = MemoryStore::new();

        let pool = SqlitePoolOptions::new().connect(&database_url).await.expect("could not connect");
        let oauth_client = config::oauth_client().unwrap();
        let app_state: AppState = AppState {
            store,
            dbreference: pool,
            oauth_client,
        };
        println!("->> Successful connection to database: {:?}", &database_url);

    //Init App Router
        let app_router =    Router::new()
            .route("/", get(handlers::root))
            .layer(middleware::from_fn_with_state(app_state.clone(), auth))
            .route("/users", get(handlers::users))
            .route("/login", get(login))
            .route("/auth/authorized", get(login_authorized))
            .fallback(handlers::error_404)
            .with_state(app_state);
    //Launch Server
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", environment_variables.address, environment_variables.port))
            .await
            .unwrap();
        println!("->> LISTENING on {:?}\n", listener.local_addr());
        axum::serve(listener, app_router)
            .await
            .unwrap();
}
pub(crate) async fn login_authorized(
    cookie_jar: CookieJar,
    State(store): State<MemoryStore>,
    State(state): State<AppState>,
    State(oauth_client): State<BasicClient>,
    Query(query): Query<AuthRequest>
) -> Result<Response, AppError> {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await?;
    // Fetch user data from Google

    let client = reqwest::Client::new();
    let user_data = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<GoogleUser>()
        .await
        .context("failed to deserialize response as JSON")?;

    // println!("{:?}", state.dbreference.get_user_by_name(&user_data.name).await?);
    let user_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM GoogleUsers WHERE name = $1)")
            .bind(&user_data.name)
            .fetch_one(&state.dbreference)
            .await
            .context("failed in finding if user exists").unwrap();

    if user_exists {
        let user = state.dbreference.get_user_by_name(&user_data.name).await?;
        let jar = JWT::create_jwt_token(state, user.id.unwrap()).await?;
        return Ok((jar,Redirect::to("/")).into_response())
    }

    let user = GoogleUser {
        id: Some(1),
        sub: user_data.sub,
        picture: user_data.picture,
        email: user_data.email,
        name: user_data.name,
    };

    let user_id = state.dbreference.create_user(user).await?;
    let jar = JWT::create_jwt_token(state, user_id).await?;

    Ok((jar, Redirect::to("/")).into_response())
}

pub(crate) async fn login(State(client): State<BasicClient>) -> impl IntoResponse {
    // TODO: this example currently doesn't validate the CSRF token during login attempts. That
    // makes it vulnerable to cross-site request forgery. If you copy code from this example make
    // sure to add a check for the CSRF token.
    // Issue for adding check to this example https://github.com/tokio-rs/axum/issues/2511
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    Redirect::to(&auth_url.to_string())
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct AuthRequest {
    code: String,
    state: String,
}
pub struct AuthRedirect;
impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/login").into_response()
    }
}
#[derive(Debug)]
pub(crate) struct AppError(anyhow::Error);
// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}
// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
}
