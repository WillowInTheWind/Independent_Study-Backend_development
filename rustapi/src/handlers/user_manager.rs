use axum::http::StatusCode;
use axum::Json;
use axum::response::Response;
use sqlx::{Pool, Sqlite};
use crate::{handlers};
use crate::handlers::GenericUser;

pub(crate) trait UserService: Send + Sync {
    fn get_users(&self) -> Response<Json<GenericUser>>;
    fn get_user_by_id(&self) -> Response<Json<GenericUser>>;
    fn get_user_by_name(&self) -> Response<Json<GenericUser>>;
    fn create_user(&self) -> (Json<GenericUser>, StatusCode);
    fn delete_user(&self) -> (Json<GenericUser>, StatusCode);
    fn edit_username(&self) ->  (Json<GenericUser>, StatusCode);
}
impl UserService for Pool<Sqlite> {
    fn get_users(&self) -> Response<Json<GenericUser>> {
        todo!()
    }

    fn get_user_by_id(&self) -> Response<Json<GenericUser>> {
        todo!()
    }

    fn get_user_by_name(&self) -> Response<Json<GenericUser>> {
        todo!()
    }

    fn create_user(&self) -> (Json<GenericUser>, StatusCode) {
        todo!()
    }

    fn delete_user(&self) -> (Json<GenericUser>, StatusCode) {
        todo!()
    }

    fn edit_username(&self) -> (Json<GenericUser>, StatusCode) {
        todo!()
    }
}
