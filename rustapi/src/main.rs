mod handlers;
mod state;
mod errors;

use sqlx::{Connection};
use axum::{routing::{get, post}, http::StatusCode, response::IntoResponse, Router};
use std::sync::Arc;
use axum::extract::{ FromRef};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use crate::handlers::user_manager::UserService;
use crate::state::{InternalState};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>  {
    tracing_subscriber::fmt::init();


    // Database Initialization
    let dburl = "C:\\Users\\Chase Wayland\\Independent_Study-Backend_development\\rustapi\\Database\\identifier.sqlite";
    let pool = SqlitePoolOptions::new().connect(dburl).await?;
    // Let's assume that an Err from func_result should end programme.

    println!("->> Successful connection to database: {:?}", dburl);
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

    Ok(())
}


