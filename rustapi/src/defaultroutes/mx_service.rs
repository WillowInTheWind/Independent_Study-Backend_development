use axum::http::StatusCode;
use axum_macros::debug_handler;
use chrono::{NaiveDate};
use sqlx::{Pool, Sqlite};
use crate::defaultroutes::user_manager::UserService;
use crate::types;
use crate::types::{ MorningExercise};
pub trait MxService {
    async fn get_mx_by_id(&self, id:i64) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_date(&self, date: NaiveDate) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_index(&self, index: i64) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_title(&self, title: &str) -> Result<types::MorningExercise, (StatusCode, String)>;
    async fn get_mxs_by_owner(&self, owner_id: i64) -> Result<Vec<types::MorningExercise>, (StatusCode, String)>;
    async fn get_mxs(&self) ->Result<Vec<types::MorningExercise>, (StatusCode, String)>;
    async fn create_mx(&self, mx: MorningExercise) -> StatusCode;
    async fn delete_mx_by_id(&self, id: i64) -> StatusCode;
    async fn delete_mx_by_index(&self, index: i64) -> StatusCode;
    async fn delete_mx_by_title(&self, title: &str) -> StatusCode;
    async fn edit_mx(&self) ->  StatusCode;
}
impl MxService for Pool<Sqlite> {
    async fn get_mx_by_id(&self, id: i64) -> Result<types::MorningExercise, (StatusCode, String)> {
        let query : Result<(i64, i64, i64, NaiveDate, String, String), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE id = ?")
            .bind(id)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4,query.5, None)
            }
            Err(e) => {
                return Err((StatusCode::NOT_FOUND, "No such MXs".to_string()))
            }
        };
        Ok(mx)
    }
    async fn get_mx_by_date(&self, date: NaiveDate) -> Result<types::MorningExercise, (StatusCode, String)> {
        let query : Result<(i64, i64, i64, NaiveDate, String, String), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE date = ?")
            .bind(date)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4,query.5, None)
            }
            Err(query) => {
                return Err((StatusCode::NOT_FOUND, "No such MXs".to_string()))
            }
        };
        Ok(mx)
    }
    async fn get_mx_by_index(&self, index: i64) -> Result<types::MorningExercise, (StatusCode, String)> {
        let query : Result<(i64, i64, i64, NaiveDate, String, String), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE mx_index = ?")
            .bind(index)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4,query.5, None)
            }
            Err(query) => {
                return Err((StatusCode::NOT_FOUND, "No such MXs".to_string()))
            }
        };
        Ok(mx)
    }
    async fn get_mx_by_title(&self, title: &str) -> Result<types::MorningExercise, (StatusCode, String)> {
        let query : Result<(i64, i64, i64, NaiveDate, String, String), _> = sqlx::query_as
            ("SELECT * FROM MX WHERE title = ?")
            .bind(title)
            .fetch_one(self)
            .await;

        let mx = match query {
            Ok(query) => {
                let user = self.get_user_by_id(query.1 as i32)
                    .await
                    .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                MorningExercise::new(query.0,user,query.2,query.3,query.4,query.5, None)
            }
            Err(query) => {
                return Err((StatusCode::NOT_FOUND, "No such MXs".to_string()))
            }
        };
        Ok(mx)
    }
    async fn get_mxs_by_owner(&self, owner_id: i64) -> Result<Vec<types::MorningExercise>, (StatusCode, String)> {
        let query : Result<Vec<(i64, i64, i64, NaiveDate, String, String)>, _> = sqlx::query_as
            ("SELECT * FROM MX where owner = ? ")
            .bind(owner_id)
            .fetch_all(self)
            .await;

        let mxs = match query {
            Ok(query) => {
                let mut mxs: Vec<MorningExercise> = Vec::new();
                for mx in query {
                    let user = self.get_user_by_id(mx.1 as i32)
                        .await
                        .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
                    mxs.push(MorningExercise::new(mx.0,user,mx.2,mx.3,mx.4,mx.5, None));
                }
                mxs
            }
            Err(query) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "Query Failed".to_string()))
            }
        };
        Ok(mxs)

    }

    async fn get_mxs(&self) -> Result<Vec<types::MorningExercise>, (StatusCode, String)> {
        let query : Vec<mxquery> = sqlx::query_as!
            (mxquery, "SELECT * FROM MX")
            .fetch_all(self)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, "query failed".to_string()))?;

        // println!("dfs");

        let mut mxs: Vec<MorningExercise> = Vec::new();
        for mx in query {
            let user = self.get_user_by_id(mx.id as i32)
                .await
                .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, "GetUserFailed".to_string()))?;
            mxs.push(MorningExercise::new(mx.id ,user,mx.mx_index,mx.date,mx.title,mx.description, None));
        }



        Ok(mxs)
    }
    async fn create_mx(&self, mx: MorningExercise) -> StatusCode {
        let index: i64 = mx.mx_index;
        let date: NaiveDate = mx.date ;
        let owner: i64 = mx.owner.id.unwrap();
        let title: String = mx.title;
        let description: String = mx.description;

        let query = sqlx::query("INSERT into MX (mx_index,date,owner, title,description) values (?,?,?,?,?)")
            .bind(index)
            .bind(date)
            .bind(owner)
            .bind(title)
            .bind(description)
            .execute(self)
            .await
            .map_err(|err| println!("{}", err));

        match query {
            Ok(q) => {
                StatusCode::CREATED
            }
            Err(q) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
    async fn delete_mx_by_id(&self, id: i64) -> StatusCode {
       let query = sqlx::query("Delete FROM MX WHERE id = ?")
            .bind(id)
            .fetch_one(self)
            .await
            .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)
            ;

        match query {
            Ok(q) => {
                StatusCode::OK
            }
            Err(q) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
         }

    }
    async fn delete_mx_by_index(&self, index: i64) -> StatusCode {
        let query = sqlx::query("Delete FROM MX WHERE id = ?")
            .bind(index)
            .fetch_one(self)
            .await
            .map_err(|err| StatusCode::INTERNAL_SERVER_ERROR)
            ;

        match query {
            Ok(q) => {
                StatusCode::OK
            }
            Err(q) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
    async fn delete_mx_by_title(&self, title: &str) -> StatusCode {
        let query = sqlx::query("Delete FROM MX WHERE id = ?")
            .bind(title)
            .fetch_one(self)
            .await
            .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)
            ;

        match query {
            Ok(q) => {
                StatusCode::OK
            }
            Err(q) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
    async fn edit_mx(&self) -> StatusCode {
        todo!()
    }
}

#[derive(Debug)]
struct mxquery {
    id: i64,
    mx_index: i64,
    owner: i64,
    date: NaiveDate,
    title: String,
    description: String
}