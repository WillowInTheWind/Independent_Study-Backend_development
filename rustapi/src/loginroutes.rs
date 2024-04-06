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
use crate::{AppError, jwt};
use crate::defaultroutes::user_manager::UserService;
use crate::state::AppState;
use crate::types::{GoogleUser};

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
        let jar = jwt::create_jwt_token(user.id.unwrap()).await?;
        return Ok((jar,Redirect::to("/")).into_response())
    }

    let user = GoogleUser {
        id: Some(1),
        sub: user_data.sub,
        picture: user_data.picture,
        email: user_data.email,
        name: user_data.name
    };

    let user_id = state.dbreference.create_user(user).await?;
    let jar = jwt::create_jwt_token( user_id).await?;

    Ok((jar, Redirect::to("/")).into_response())
}

pub(crate) async fn logout(
) -> Result<Response, StatusCode> {
    println!("->> user logged out");

    let cookies = jwt::remove_jwt_token().await.map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((cookies, Redirect::to("/")).into_response())
}

pub(crate) async fn login(State(client): State<BasicClient>) -> Response {
    println!("->> user logged in or signed up");

    // TODO: this example currently doesn't validate the CSRF token during login attempts. That
    // makes it vulnerable to cross-site request forgery. If you copy code from this example make
    // sure to add a check for the CSRF token.
    // Issue for adding check to this example https://github.com/tokio-rs/axum/issues/2511

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    let url = auth_url.to_string();
    let authurl = Json(url);
    authurl.into_response()
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
