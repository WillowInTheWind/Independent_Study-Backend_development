use axum::http::StatusCode;
use axum::Json;
use sqlx::{Pool, Sqlite};
use crate::handlers::GenericUser;
use crate::errors::QueryErrors;

pub(crate) trait UserService: Send + Sync {
    fn get_users(&self) -> Result<Vec<Json<GenericUser>>, sqlx::Error>;
    fn get_user_by_id(&self) -> Result<Json<GenericUser>, (StatusCode, String)>;
    fn get_user_by_name(&self, name:&str) -> Result<Json<GenericUser>, (StatusCode, String)>;
    fn create_user(&self) -> Result<Json<GenericUser>, (StatusCode, String)>;
    fn delete_user(&self) -> Result<Json<GenericUser>, (StatusCode, String)>;
    fn edit_username(&self) ->  Result<Json<GenericUser>, (StatusCode, String)>;
}
impl UserService for Pool<Sqlite> {
    async fn get_users(&self) -> Result<Vec<Json<GenericUser>>, sqlx::Error> {
        sqlx::query(todo!())
            .fetch_all(&self)
            .await
    }

    fn get_user_by_id(&self) -> Result<Json<GenericUser>, (StatusCode, String)> {
        todo!()
    }

    fn get_user_by_name(&self, name: &str) -> Result<Json<GenericUser>, (StatusCode, String)> {
        todo!()
    }

    fn create_user(&self) -> Result<Json<GenericUser>, (StatusCode, String)> {
        todo!()
    }

    fn delete_user(&self) -> Result<Json<GenericUser>, (StatusCode, String)> {
        todo!()
    }

    fn edit_username(&self) -> Result<Json<GenericUser>, (StatusCode, String)> {
        todo!()
    }
}
