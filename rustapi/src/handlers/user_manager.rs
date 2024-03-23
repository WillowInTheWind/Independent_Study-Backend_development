use axum::http::StatusCode;
use axum::Json;
use sqlx::{Sqlite, Pool, FromRow, Error};
use crate::handlers::GenericUser;
use crate::errors::QueryErrors;

pub(crate) trait UserService: Send + Sync {
    async fn get_users(&self) -> Result<Vec<GenericUser>, sqlx::Error>;
    async fn get_user_by_id(&self, id: i32) -> Result<GenericUser, sqlx::Error>;
    async fn get_user_by_name(&self, name:&str) -> Result<GenericUser, sqlx::Error>;
    async fn create_user(&self) -> Result<GenericUser, sqlx::Error>;
    async fn delete_user(&self) -> Result<GenericUser, sqlx::Error>;
    async fn edit_username(&self) ->  Result<GenericUser, sqlx::Error>;
}
impl UserService for Pool<Sqlite> {
    async fn get_users(&self) -> Result<Vec<GenericUser>, sqlx::Error> {
        let query: Result<Vec<GenericUser>, sqlx::Error> =
            sqlx::query_as(
            "SELECT id, user_name, user_identifier FROM user")
            .fetch_all(self)
            .await
            ;

        query
    }

    async fn get_user_by_id(&self, id:i32) -> Result<GenericUser, sqlx::Error> {
        let query: Result<GenericUser, sqlx::Error> =
            sqlx::query_as!(
                GenericUser,
                "SELECT * FROM user WHERE id = $1",
                id
            )
                .fetch_one(self)
                .await
            ;

        query
    }

    async fn get_user_by_name(&self, name: &str) -> Result<GenericUser, Error> {
        let query: Result<GenericUser, sqlx::Error> =
            sqlx::query_as!(
                GenericUser,
                "SELECT * FROM user WHERE user_name = $1",
                name
                )
                .fetch_one(self)
                .await
            ;

        query
    }

    async fn create_user(&self) -> Result<GenericUser, Error> {
        todo!()
    }

    async fn delete_user(&self) -> Result<GenericUser, Error> {
        todo!()
    }

    async fn edit_username(&self) -> Result<GenericUser, Error> {
        todo!()
    }
}
