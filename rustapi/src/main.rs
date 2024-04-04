mod defaultroutes;
mod state;
mod mx_date_algorithm;
mod types;
mod config;
mod middlewares;
mod jwt;
mod loginroutes;
mod MXroutes;
mod UserRoutes;

use oauth2::TokenResponse;
use axum::{middleware, Router, routing::get};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use dotenv::dotenv;
use anyhow::Context;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::defaultroutes::user_manager::UserService;
use crate::middlewares::auth;
use crate::state::AppState;

//TODO: Add CSRF token Validation
//TODO: Consolidate Error Types
#[tokio::main]
async fn main(){
    //Init enviorment variables and tracing
        dotenv().ok();
        tracing_subscriber::fmt::init();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must set");
        let environment_variables = config::initialize_environment_variable().await;
    //Init App State
        let pool = SqlitePoolOptions::new().connect(&database_url).await.expect("could not connect");
        let oauth_client = config::oauth_client().unwrap();
        let app_state: AppState = AppState {
            dbreference: pool,
            oauth_client,
        };

        println!("->> Successful connection to database: {:?}", &database_url);
    //Init App router
        let app_router = router(app_state);
    //Launch Server
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", environment_variables.address, environment_variables.port))
            .await
            .unwrap();
        println!("->> LISTENING on {:?}\n", listener.local_addr());
        axum::serve(listener, app_router)
            .await
            .unwrap();
}

fn router(app_state: AppState) -> Router {
    let mx_routes = Router::new()
        .route("/", get(MXroutes::get_all_mxs))
        .route("/create", post(MXroutes::post_mx))
        .route("/mine", get(MXroutes::get_users_mxs));
    let user_routes = Router::new()
        .route("/", get(UserRoutes::get_all_users))
        .route("/:id", get(UserRoutes::get_user_by_id));
    let auth_routes = Router::new()
        .route("/logout", get(loginroutes::logout))
        .route("/login", get(loginroutes::login))
        .route("/authorized", get(loginroutes::login_authorized));

    Router::new()
        .route("/", get(defaultroutes::root))
        .nest("/morningexercises", mx_routes)
        .nest("/users", user_routes)
        .layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .nest("/auth", auth_routes)
        .fallback(defaultroutes::error_404)
        .with_state(app_state)
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
