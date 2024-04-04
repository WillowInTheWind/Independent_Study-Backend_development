use sqlx::{Sqlite, Pool, Error};
use crate::types::{ GoogleUser};
pub(crate) trait UserService: Send + Sync {
    async fn get_users(&self) -> Result<Vec<GoogleUser>, sqlx::Error>;
    async fn get_user_by_id(&self, id: i32) -> Result<GoogleUser, sqlx::Error>;
    async fn get_user_by_name(&self, name:&str) -> Result<GoogleUser, sqlx::Error>;
    async fn get_user_by_sub(&self, sub:&str) -> Result<GoogleUser, sqlx::Error>;
    async fn get_user_by_email(&self, email:&str) -> Result<GoogleUser, sqlx::Error>;

    async fn create_user(&self, new_user: GoogleUser) -> Result<i64, sqlx::Error>;

    async fn delete_user_by_id(&self, id: i32) -> Result<i64, sqlx::Error>;
    async fn delete_user_by_user_name(&self, name: String) -> Result<i64, sqlx::Error>;

    async fn edit_username(&self, new_user: GoogleUser) ->  Result<GoogleUser, sqlx::Error>;
    async fn delete_user_by_email(&self, email: String) -> Result<i64, Error>;
}
impl UserService for Pool<Sqlite> {
    async fn get_users(&self) -> Result<Vec<GoogleUser>, Error> {
        let query = sqlx::query_as!(
            GoogleUser, "SELECT * FROM GoogleUsers")
            .fetch_all(self).await;
        query
    }
    async fn get_user_by_id(&self, id: i32) -> Result<GoogleUser, Error> {
        let query = sqlx::query_as!(
            GoogleUser, "SELECT * FROM GoogleUsers Where id = $1", id)
            .fetch_one(self).await;
        query
    }

    async fn get_user_by_name(&self, name: &str) -> Result<GoogleUser, Error> {
        let query = sqlx::query_as!(
            GoogleUser, "SELECT * FROM GoogleUsers Where name = $1", name)
            .fetch_one(self).await;
        query
    }

    async fn get_user_by_sub(&self, sub: &str) -> Result<GoogleUser, Error> {
        let query = sqlx::query_as!(
            GoogleUser, "SELECT * FROM GoogleUsers Where sub = $1", sub)
            .fetch_one(self).await;
        query
    }

    async fn get_user_by_email(&self, email: &str) -> Result<GoogleUser, Error> {
        let query = sqlx::query_as!(
            GoogleUser, "SELECT * FROM GoogleUsers Where email = $1", email)
            .fetch_one(self).await;
        query
    }

    async fn create_user(&self, new_user: GoogleUser) -> Result<i64, Error> {
        let query =
            sqlx::query!(
                "INSERT into GoogleUsers (sub, picture, email, name) values ($1,$2, $3, $4)",
                new_user.sub,
                new_user.picture,
                new_user.email,
                new_user.name
            )
                .execute(self)
                .await?
                .last_insert_rowid()
            ;
        Ok(query)
    }

    async fn delete_user_by_id(&self, id: i32) -> Result<i64, Error> {
        let query =
            sqlx::query!(
                "Delete from GoogleUsers where id = $1",
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
                "Delete from GoogleUsers where name = $1",
                name
            )
                .execute(self)
                .await?
                .last_insert_rowid()
            ;
        Ok(query)
    }
    async fn edit_username(&self, new_user: GoogleUser) -> Result<GoogleUser, Error> {
        todo!()
    }
    async fn delete_user_by_email(&self, email: String) -> Result<i64, Error> {
        let query =
            sqlx::query!(
                "Delete from GoogleUsers where email = $1",
                email
            )
                .execute(self)
                .await?
                .last_insert_rowid()
            ;
        Ok(query)
    }
}
