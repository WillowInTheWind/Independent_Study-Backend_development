mod handlers;
mod state;
mod mxdate_algorithim;
use sqlx::{Connection};
use axum::{routing::{get, post}, response::IntoResponse, Router};
use std::sync::Arc;
use axum::extract::{ FromRef};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use crate::handlers::user_manager::UserService;
use crate::state::{InternalState};
use std::env;
use std::error::Error;
use std::future::IntoFuture;
use axum_macros::debug_handler;

// #[debug_handler]
struct EnvironmentVariables {
    app_state: Arc<InternalState>,
    address: String,
    port: String,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let environment_variables = initialize_environment_variable().await;
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", environment_variables.address, environment_variables.port))
        .await
        .unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app_router(environment_variables))
        .await
        .unwrap();
}
async fn initialize_environment_variable() -> EnvironmentVariables {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = SqlitePoolOptions::new().connect(&database_url).await.expect("could not connect");
    println!("->> Successful connection to database: {:?}", &database_url);
    let app_state: Arc<InternalState> =  Arc::new(
        InternalState::new(pool)
    );
    let address: String = match env::var("ADDRESS") {
        Ok(address) => {address}
        None => {"127.0.0.1"}
    };
    let port: String = match env::var("PORT") {
        Ok(port) => { port }
        None => {"8080"}
    };

    EnvironmentVariables {
        app_state,
        address,
        port,
    }
}
fn app_router (environment_variables: EnvironmentVariables) -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/users", get(handlers::users))
        .with_state(environment_variables.app_state)
}