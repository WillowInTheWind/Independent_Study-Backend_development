use std::env::VarError;
use axum::response::{Html};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use crate::mx_date_algorithm;

pub(crate) trait DateToString {
    fn date_to_long_string(&self) -> String;
    fn date_to_short_string(&self) -> String;

}
impl DateToString for NaiveDate {
    fn date_to_long_string(&self) -> String {
        let month = match self.month() {
            1 => { "January" }
            2 => { "Febuary" }
            3 => { "March" }
            4 => { "April" }
            5 => {"May"}
            6 => {"June"}
            7 => {"July"}
            8 => {"August"}
            9 => {"September"}
            10 => {"October"}
            11 => {"November"}
            12 => {"December"}
            _ => {"Unreachable value"}
        };
        let day = match self.day() {
            1 => {format!("{:?}st",self.day())}
            2 => {format!("{:?}nd",self.day())}
            3 => {format!("{:?}rd",self.day())}
            _ => {format!("{:?}th",self.day())}
        };
        format!("{} {} {}",month, self.day(), self.year()%1000)

    }
    fn date_to_short_string(&self) -> String {
        format!("{}/{}/{}",self.month(), self.day(), self.year()%1000)
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MorningExercise {
    id: i32,
    pub(crate) mx_index: i32,
    pub(crate) date:  chrono::NaiveDate,
    pub(crate) owner: GoogleUser,
    pub(crate) title: String,
    pub(crate) description: String,
    // editors: Vec<GenericUser>
}
impl MorningExercise {
    //constructors
    pub fn new_with_index(id:i32,
                          owner: GoogleUser,
                          mx_index: i32,
                          title: String,
                          description: String,
                          editors: Vec<GenericUser>)
                          -> Self {
        MorningExercise {
            id,
           mx_index,
            date: mx_date_algorithm::weekly_index_to_date(),
            owner,
            title ,
            description ,
        }
    }
    pub fn new_with_date(id:i32,
                         owner: GoogleUser,
                         date: NaiveDate,
                         title: String,
                         description: String,
                         editors: Option<Vec<GenericUser>>)
                         -> Self {
        MorningExercise {
            id,
            date,
            mx_index: mx_date_algorithm::weekly_date_to_index() as i32,
            owner,
            title ,
            description ,
        }
    }
    pub fn new(id:i32,
               owner: GoogleUser,
               mx_index: i32,
               date: NaiveDate,
               title: String,
               description: String,
               editors: Option<GenericUser>)
                          -> Self {
        MorningExercise {
            id,
            mx_index,
            date,
            owner,
            title ,
            description ,
        }
    }
}
#[derive(FromRow, Debug, Deserialize, Serialize, Clone)]
#[warn(deprecated)]
pub(crate) struct GenericUser {
    pub(crate) id: Option<i64>,
    pub(crate) name: String,
    pub(crate) user_identifier: i64,
    pub(crate) user_email: String
}
#[derive(FromRow, Debug, Deserialize, Serialize, Clone)]
pub(crate) struct  GoogleUser {
    pub(crate) id: Option<i32>,
    pub(crate) sub: String,
    pub(crate) picture: Option<String>,
    pub(crate) email: String,
    pub(crate) name: String,
    pub(crate) token: Option<String>,
    pub(crate) phone_number: Option<String>
    // pub(crate) isAdmin: bool,
}
/// Errors, there are too many of them
/// TODO: consolidate error type into one solid type
pub(crate) struct AppError(pub anyhow::Error);
pub fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

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

#[derive(Debug,Deserialize,Serialize)]
pub struct CalendarEvent {
    summary: String,
    start: time,
    end: time,
    description: String
}

impl CalendarEvent {
    pub fn new(summary: String, start: NaiveDateTime, end: NaiveDateTime, description: String) -> Self {
        CalendarEvent{
            summary,
            start: time {
                dateTime: start, timeZone: "America/Chicago".to_string()
            },
            end: time {
                dateTime: end, timeZone: "America/Chicago".to_string()
            },
            description,
        }
    }
}
#[derive(Debug,Deserialize,Serialize)]
struct time {
    dateTime: NaiveDateTime,
    timeZone: String
}