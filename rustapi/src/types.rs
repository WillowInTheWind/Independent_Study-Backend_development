use time::Date;
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::mxdate_algorithim;

pub(crate) trait DateToString {
    fn date_to_long_string(&self) -> String;
    fn date_to_short_string(&self) -> String;

}
impl DateToString for Date {
    fn date_to_long_string(&self) -> String {
        todo!()
    }

    fn date_to_short_string(&self) -> String {
        todo!()
    }
}
pub(crate) struct MorningExercise {
    pub(crate) id: u32,
    pub(crate) index:u16,
    pub(crate) date: Date,
    pub(crate) owner: GenericUser,
    pub(crate) title: String,
    pub(crate) description: String,
    // editors: Vec<GenericUser>
}
impl MorningExercise {

    //constructors
    pub fn new_with_index(id:u32,
                          owner: GenericUser,
                          index: u16,
                          title: String,
                          description: String,
                          editors: Vec<GenericUser>)
                          -> Self {
        MorningExercise {
            id,
            index,
            date: mxdate_algorithim::weekly_index_to_date(),
            owner,
            title ,
            description ,
        }
    }
    pub fn new_with_date(id:u32,
                         owner: GenericUser,
                         date: Date,
                         title: String,
                         description: String,
                         editors: Vec<GenericUser>)
                         -> Self {
        MorningExercise {
            id,
            date,
            index: mxdate_algorithim::weekly_date_to_index(),
            owner,
            title ,
            description ,
        }
    }
}

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub(crate) struct GenericUser {
    pub(crate) id: Option<i64>,
    pub(crate) user_name: String,
    //if I end up implementing other way to login besides google Oauth I can change the user type
    pub(crate) user_identifier: i64,
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
#[derive(Debug)]
pub(crate) struct AppError(anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}
