use axum::http::StatusCode;
use chrono::{NaiveDate};
use sqlx::{Pool, Sqlite};
use crate::handlers::user_manager::UserService;
use crate::types;
use crate::types::{ MorningExercise};

trait MxService {
    async fn get_mx_by_id(&self) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_date(&self) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_index(&self) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_title(&self) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_owner(&self, owner_id: i64) -> Result<types::MorningExercise, (StatusCode, String)>;
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
    async fn get_mx_by_owner(&self, owner_id: i64) -> Result<types::MorningExercise, (StatusCode, String)> {
        let query : Result<(i64, i64, i64, NaiveDate, String, String), _> = sqlx::query_as
        ("SELECT * FROM MX WHERE owner = ?")
            .bind(owner_id)
            .fetch_one(self)
            .await;

        let mx = match query {

            Ok(query) => {
                let user = self.get_user_by_id(query.1 as i32).await.map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4,query.5, None)
            }
            Err(query) => {
                return Err((StatusCode::NOT_FOUND, "No such MXs".parse().unwrap()))
            }
        };
        Ok(mx)
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