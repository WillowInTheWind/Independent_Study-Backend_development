
mod handlers;
mod state;
mod errors;

use sqlx::{Connection, Sqlite, SqlitePool};
use axum::{routing::{get, post}, http::StatusCode, response::IntoResponse, Router};
use std::sync::Arc;
use axum::extract::{ FromRef};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use crate::handlers::user_manager::UserService;
use crate::state::{InternalState};
use dotenv::dotenv;
use std::error::Error;
use std::future::IntoFuture;
use axum_macros::debug_handler;


// #[debug_handler]
#[tokio::main]
async fn main() {
    // dotenv()?;
    dotenv().expect("TODO: panic message");

    tracing_subscriber::fmt::init();


    // Database Initialization
    let dburl = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = SqlitePoolOptions::new().connect(&dburl).await.expect("could not connect");
    // Let's assume that an Err from func_result should end programme.

    println!("->> Successful connection to database: {:?}", &dburl);
    //Setting up Server
    let app_state: Arc<InternalState> =  Arc::new(
            InternalState::new(pool)
    );

    let app_router = Router::new()
        .route("/", get(handlers::root))
        .route("/login", get(handlers::login))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());

    axum::serve(listener, app_router)
        .await
        .unwrap();
}


