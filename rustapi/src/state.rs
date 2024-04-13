use axum::extract::FromRef;
use sqlx::{Pool, Postgres};
use oauth2::{
    basic::BasicClient
};
//Define AppState
#[derive(Clone)]
pub struct AppState {
    pub dbreference: Pool<Postgres>,
    pub(crate) oauth_client: BasicClient,
    pub reqwestClient: reqwest::Client,
}
impl AppState {
    pub fn new(db:  Pool<Postgres>, oauth_client: BasicClient, client: reqwest::Client) -> Self {
        AppState {
            dbreference: db,
            oauth_client,
            reqwestClient: client,
        }
    }
}
impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}
