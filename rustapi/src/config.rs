use std::env;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use anyhow::Context;
use crate::AppError;

pub struct EnvironmentVariables {
    pub address: String,
    pub port: String,
}
pub async fn initialize_environment_variable() -> EnvironmentVariables {
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
pub(crate) fn oauth_client() -> anyhow::Result<BasicClient, AppError> {
    dotenv::dotenv().ok();
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
