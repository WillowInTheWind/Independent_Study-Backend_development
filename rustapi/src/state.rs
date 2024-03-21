use sqlx::{Pool, Sqlite};

pub struct InternalState {
    pub dbreference: Pool<Sqlite>,
}

impl InternalState {
    pub fn new(db: Pool<Sqlite>) -> Self {
        InternalState{
            dbreference: db,
        }
    }
}

pub type AppState = std::sync::Arc<InternalState>;