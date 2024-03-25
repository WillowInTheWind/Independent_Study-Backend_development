use axum::http::StatusCode;
use sqlx::{Pool, Sqlite};
use crate::types;

trait MxService {
    async fn get_mx_by_id(&self) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_date(&self) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_index(&self) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_title(&self) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_owner(&self) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mxs(&self) ->Result<Vec<types::MorningExercise>, (StatusCode, String)>;
    async fn create_mx(&self) -> StatusCode;
    async fn delete_mx_by_id(&self) -> StatusCode;
    async fn delete_mx_by_index(&self) -> StatusCode;
    async fn delete_mx_by_title(&self) -> StatusCode;
    async fn edit_mx(&self) ->  StatusCode;
}

impl MxService for Pool<Sqlite> {
    async fn get_mx_by_id(&self) -> Result<types::MorningExercise, (StatusCode, String)> {
        todo!()
    }
    async fn get_mx_by_date(&self) -> Result<types::MorningExercise, (StatusCode, String)> {
        todo!()
    }
    async fn get_mx_by_index(&self) -> Result<types::MorningExercise, (StatusCode, String)> {
        todo!()
    }
    async fn get_mx_by_title(&self) -> Result<types::MorningExercise, (StatusCode, String)> {
        todo!()
    }
    async fn get_mx_by_owner(&self) -> Result<types::MorningExercise, (StatusCode, String)> {
        todo!()
    }
    async fn get_mxs(&self) -> Result<Vec<types::MorningExercise>, (StatusCode, String)> {
        todo!()
    }
    async fn create_mx(&self) -> StatusCode {
        todo!()
    }
    async fn delete_mx_by_id(&self) -> StatusCode {
        todo!()
    }
    async fn delete_mx_by_index(&self) -> StatusCode {
        todo!()
    }
    async fn delete_mx_by_title(&self) -> StatusCode {
        todo!()
    }
    async fn edit_mx(&self) -> StatusCode {
        todo!()
    }
}