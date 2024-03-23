use axum::http::StatusCode;
use axum::Json;
use sqlx::{Sqlite, Pool, FromRow};
use crate::handlers::GenericUser;
use crate::errors::QueryErrors;

pub(crate) trait UserService: Send + Sync {
    async fn get_users(&self) -> Result<Vec<Json<GenericUser>>, sqlx::Error>;
    fn get_user_by_id(&self) -> Result<Json<GenericUser>, (StatusCode, String)>;
    fn get_user_by_name(&self, name:&str) -> Result<Json<GenericUser>, (StatusCode, String)>;
    fn create_user(&self) -> Result<Json<GenericUser>, (StatusCode, String)>;
    fn delete_user(&self) -> Result<Json<GenericUser>, (StatusCode, String)>;
    fn edit_username(&self) ->  Result<Json<GenericUser>, (StatusCode, String)>;
}
impl UserService for Pool<Sqlite> {
    async fn get_users(&self) -> Result<Vec<Json<GenericUser>>, sqlx::Error> {
        let query: Vec<_> = sqlx::query(
            "SELECT id, user_name, user_identifier FROM user"
        ).fetch_all(&self)
            .await.unwrap()
            .into_iter()
            .map(|row| GenericUser::from_row(&row))
            .collect();

        for response in query {
            println!("{:?}", response.unwrap());
            // let User = GenericUser {
            //     id: response.clone().unwrap().id,
            //     user_name: response.clone().unwrap().username,
            //     user_identifier: response.clone().unwrap().UniqueIndentifier,
            // }
        }
        todo!()
        // query
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
