use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router};
use axum::response::{Html, Response};

use std::net::SocketAddr;
use axum::extract::Query;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    let routes_hello = Router::new().merge(routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());

    axum::serve(listener, routes_hello)
        .await
        .unwrap();
}
fn routes() -> Router {
    Router::new().route("/hello",get(handler_hello))
}
async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}<strong>"))
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}