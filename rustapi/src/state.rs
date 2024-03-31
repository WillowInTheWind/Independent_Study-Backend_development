use async_session::MemoryStore;
use axum::extract::FromRef;
use sqlx::{Pool, Sqlite};
use oauth2::{
    basic::BasicClient
};


//Define AppState
#[derive(Clone)]
pub struct AppState {
    pub store: MemoryStore,
    pub dbreference: Pool<Sqlite>,
    pub(crate) oauth_client: BasicClient
}
impl AppState {
    pub fn new(db: Pool<Sqlite>, oauth_client: BasicClient, store: MemoryStore) -> Self {
        AppState {
            store,
            dbreference: db,
            oauth_client
        }
    }
}
impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}
impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}
