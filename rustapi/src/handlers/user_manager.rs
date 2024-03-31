// use axum::http::StatusCode;
use sqlx::{Sqlite, Pool, Error};
use crate::types::GenericUser;
pub(crate) trait UserService: Send + Sync {
    async fn get_users(&self) -> Result<Vec<GenericUser>, sqlx::Error>;
    async fn get_user_by_id(&self, id: i32) -> Result<GenericUser, sqlx::Error>;
    async fn get_user_by_name(&self, name:&str) -> Result<GenericUser, sqlx::Error>;
    async fn create_user(&self, new_user: GenericUser) -> Result<i64, sqlx::Error>;
    async fn delete_user_by_id(&self, id: i32) -> Result<i64, sqlx::Error>;
    async fn delete_user_by_user_name(&self, name: String) -> Result<i64, sqlx::Error>;

    async fn edit_username(&self, new_user: GenericUser) ->  Result<GenericUser, sqlx::Error>;
}
impl UserService for Pool<Sqlite> {
    //get methods
    async fn get_users(&self) -> Result<Vec<GenericUser>, sqlx::Error> {
        let query: Result<Vec<GenericUser>, sqlx::Error> =
            sqlx::query_as!(GenericUser,
            "SELECT id, name, user_identifier,user_email FROM user")
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
                "SELECT * FROM user WHERE name = $1",
                name
            )
                .fetch_one(self)
                .await
            ;

        query
    }
//post put and edit methods
    async fn create_user(&self, new_user: GenericUser) -> Result<i64, Error> {
    let query =
        sqlx::query!(
                // GenericUser,
                "INSERT into user (name, user_identifier, user_email) values ($1,$2, $3)",
                new_user.name,
                new_user.user_identifier,
                new_user.user_email
            )
            .execute(self)
            .await?
            .last_insert_rowid()
        ;

    Ok(query)
    // todo!()
    }
    async fn delete_user_by_id(&self, id: i32) -> Result<i64, Error> {
        let query =
            sqlx::query!(
                // GenericUser,
                r#"Delete from user where id = $1"#,
                id
            )
                .execute(self)
                .await?
                .last_insert_rowid()
            ;

        Ok(query)
    }

    async fn delete_user_by_user_name(&self, name: String) -> Result<i64, Error> {
        let query =
            sqlx::query!(
                // GenericUser,
                "Delete from user where name = $1",
                name
            )
                .execute(self)
                .await?
                .last_insert_rowid()
            ;

        Ok(query)
    }
    async fn edit_username(&self, new_user: GenericUser) -> Result<GenericUser, Error>
    {
        todo!()
    }
}
