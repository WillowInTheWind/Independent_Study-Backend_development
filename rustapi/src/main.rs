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
mod router;

use oauth2::TokenResponse;
use axum::{middleware, Router, routing::get};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use dotenv::dotenv;
use anyhow::Context;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, post};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteJournalMode::Delete;
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
        let client = reqwest::Client::new();

        let app_state: AppState = AppState {
            dbreference: pool,
            oauth_client,
            reqwestClient: client,
        };
        println!("->> Successful connection to database: {:?}", &database_url);
    //Init App router
        let app_router = router::router(app_state);
    //Launch Server
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", environment_variables.address, environment_variables.port))
            .await
            .unwrap();
        println!("->> LISTENING on {:?}\n", listener.local_addr());
        axum::serve(listener, app_router)
            .await
            .unwrap();
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
