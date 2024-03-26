mod handlers;
mod state;
mod mxdate_algorithim;
mod types;

use handlers::login_service::oauth_client;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::string::String;
use axum::{routing::{get, post}, Router, Extension};
use std::sync::Arc;
use sqlx::sqlite::SqlitePoolOptions;
use crate::state::{InternalState};
use std::env;
use crate::types::GenericUser;
// use axum_macros::debug_handler;

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
    let user_data = None;
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app_router(environment_variables, user_data))
        .await
        .unwrap();
    let user_data: Option<GenericUser> = None;

}
async fn initialize_environment_variable() -> EnvironmentVariables {
    let outhclient= oauth_client().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = SqlitePoolOptions::new().connect(&database_url).await.expect("could not connect");
    println!("->> Successful connection to database: {:?}", &database_url);
    let app_state: Arc<InternalState> =  Arc::new(
        InternalState::new(pool,outhclient)
    );
    let address: String = match env::var("ADDRESS") {
        Ok(address) => {address}
        _ => {String::from("127.0.0.1")}
    };
    let port: String = match env::var("PORT") {
        Ok(port) => { port }
        _ => {"8080".to_string()}
    };

    EnvironmentVariables {
        app_state,
        address,
        port,
    }
}


fn app_router (environment_variables: EnvironmentVariables, user_data: Option<GenericUser>) -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/users", get(handlers::users))
        .with_state(environment_variables.app_state)
        .layer(Extension(user_data))
}

