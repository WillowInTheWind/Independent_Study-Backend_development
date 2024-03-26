use sqlx::{Pool, Sqlite};
use oauth2::{
    basic::BasicClient
};
pub struct InternalState {
    pub dbreference: Pool<Sqlite>,
    pub(crate) oauth_client: BasicClient
}

impl InternalState {
    pub fn new(db: Pool<Sqlite>, oauth_client: BasicClient) -> Self {
        InternalState{
            dbreference: db,
            oauth_client
        }
    }
}
pub type AppState = std::sync::Arc<InternalState>;