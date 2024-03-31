mod handlers;
mod state;
mod mxdate_algorithim;
mod types;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::string::String;
use axum::{routing::{get, post}, Router, Extension, middleware};
use std::sync::Arc;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use axum::extract::{FromRef, FromRequestParts};
use dotenv::dotenv;
use sqlx::{Pool, Sqlite};
use anyhow::{Context, Result};
use crate::types::GoogleUser;
use async_session::{MemoryStore, Session, SessionStore};
use async_session::chrono::Utc;
use axum::{
    async_trait,
    extract::{ Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect, Response},
    RequestPartsExt,
};
use axum_extra::{headers, typed_header::TypedHeaderRejectionReason, TypedHeader};
use http::{header, request::Parts, StatusCode};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use crate::state::AppState;
use crate::types::GenericUser;
static COOKIE_NAME: &str = "SESSION";

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
        let environment_variables = initialize_environment_variable().await;
    //Init App State
        let store = MemoryStore::new();

        let pool = SqlitePoolOptions::new().connect(&database_url).await.expect("could not connect");
        let oauth_client = oauth_client().unwrap();
        let app_state: AppState = AppState {
            store,
            dbreference: pool,
            oauth_client,
        };
        println!("->> Successful connection to database: {:?}", &database_url);

    //Init App Router
        let app_router =    Router::new()
            .route("/", get(handlers::root))
            .route("/users", get(handlers::users))
            .route("/login", get(login))
            .route("/auth/authorized", get(login_authorized))
            .fallback(error_404)
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
async fn error_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

async fn initialize_environment_variable() -> EnvironmentVariables {
    let address: String = match env::var("ADDRESS") {
        Ok(address) => {address}
        _ => {String::from("127.0.0.1")}
    };
    let port: String = match env::var("PORT") {
        Ok(port) => { port }
        _ => {"8080".to_string()}
    };
    EnvironmentVariables {
        address,
        port,
    }
}
// OAUTH Code: Note to future mantainers, this code DID NOT work
// until I moved into *this* file, do not try to pull this out
// into its own file it will only make you cry, ask me how I know
//Oauth Methods
pub(crate) fn oauth_client() -> Result<BasicClient, AppError> {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let redirect_url = "http://127.0.0.1:8080/auth/authorized".to_string();
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");


    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
        .set_redirect_uri(
            RedirectUrl::new(redirect_url.to_string()).context("failed to create new redirection URL")?,
        ))
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
pub(crate) async fn login_authorized(
    State(store): State<MemoryStore>,
    State(state): State<AppState>,
    State(oauth_client): State<BasicClient>,
    Query(query): Query<AuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await?;
    // Fetch user data from Google

    let client = reqwest::Client::new();
    let user_data = client
        // https://discord.com/developers/docs/resources/user#get-current-user
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<GoogleUser>()
        .await
        .context("failed to deserialize response as JSON")?;

    let mut session = Session::new();
    session
        .insert("user", &user_data)
        .context("failed in inserting serialized value into session")?;

    // Store session and get corresponding cookie
    let cookie = store
        .store_session(session)
        .await
        .context("failed to store session")?
        .context("unexpected error retrieving cookie value")?;

    // Build the cookie
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to("/")))
}
struct AuthRedirect;
impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/discord").into_response()
    }
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct AuthRequest {
    code: String,
    state: String,
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
