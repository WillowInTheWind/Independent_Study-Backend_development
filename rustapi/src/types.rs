use std::env::VarError;
use axum::response::Html;
use axum::response::Response;
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

#[derive(FromRow, Debug, Deserialize, Serialize, Clone)]
pub(crate) struct GenericUser {
    pub(crate) id: Option<i64>,
    pub(crate) user_name: String,
    //if I end up implementing other way to login besides google Oauth I can change the user type
    pub(crate) user_identifier: i64,
    pub(crate) user_email: String
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
#[derive(Debug)]
pub(crate) struct AppError(pub anyhow::Error);


#[derive(Debug)]
pub struct OauthError {
    code: StatusCode,
    message: String,
    user_message: String,
}

impl OauthError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            user_message: "".to_owned(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn with_user_message(self, user_message: impl Into<String>) -> Self {
        Self {
            user_message: user_message.into(),
            ..self
        }
    }
    // pub fn with_code(self, code: StatusCode) -> Self {
    //     Self {
    //         code,
    //         ..self
    //     }
    // }
}
impl From<VarError> for OauthError {
    fn from(err: VarError) -> Self {
        OauthError::new(format!("Dotenv error: {:#}", err))
    }
}

impl IntoResponse for OauthError {
    fn into_response(self) -> axum::response::Response {
        println!("AppError: {}", self.message);
        (
            self.code,
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title>Oops!</title>
                </head>
                <body>
                    <h1>Oops!</h1>
                    <p>Sorry, but something went wrong.</p>
                    <p>{}</p>
                </body>
                </html>
                "#,
                self.user_message
            )),
        )
            .into_response()
    }
}


impl From<sqlx::Error> for OauthError {
    fn from(err: sqlx::Error) -> Self {
        OauthError::new(format!("Database query error: {:#}", err))
    }
}

impl From<String> for OauthError {
    fn from(err: String) -> Self {
        OauthError::new(err)
    }
}

impl From<&str> for OauthError {
    fn from(err: &str) -> Self {
        OauthError::new(err)
    }
}