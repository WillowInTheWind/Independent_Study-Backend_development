mod handlers;
mod state;

use sqlx::Sqlite;
use axum::{routing::{get, post}, http::StatusCode, response::IntoResponse, Json, Router, Extension};
use axum::response::{Html, Response};

use std::net::SocketAddr;
use std::ptr::null;
use std::sync::Arc;
use axum::extract::{Query, {State, FromRef}};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use serde_json::Value::String;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, sqlite};
use tokio::sync::Mutex;
use crate::handlers::GenericUser;
use crate::handlers::user_manager::UserService;
use crate::state::{AppState, InternalState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();


    // Database Initialization
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = SqlitePoolOptions::new()
        // .max_connections(5)
        .connect(&url)
        .await
        .unwrap();

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
// #[debug_handler]
// fn routes() -> Router {
//
//         // .route(
//         //     "/login",
//         //     get(login(axum::extract::State(state)))
//         // )
//
// }




#[debug_handler]
pub async fn root( ) -> &'static str {
    "This is the main route of the server"
}
// #[debug_handler]
// pub async fn login(
//     State(state): State<AppState>,
// ) -> Json<GenericUser> {
//     todo!()
// }
