

use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use jsonwebtoken::{DecodingKey, EncodingKey};
use crate::types::mx_date_algorithm;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MorningExercise {
    //TODO: add editor value so multiple people can edit the same mx
    id: i32,
    pub mx_index: i32,
    pub date: NaiveDate,
    pub owner: GoogleUser,
    pub title: String,
    pub description: String,
}
#[derive(FromRow, Debug, Deserialize, Serialize, Clone)]
pub struct  GoogleUser {
    pub id: Option<i32>,
    pub sub: String,
    pub picture: Option<String>,
    pub email: String,
    pub name: String,
    pub token: Option<String>,
    pub phone_number: Option<String>
}

#[derive(Debug,Deserialize,Serialize)]
pub struct CalendarEvent {
    summary: String,
    start: time,
    end: time,
    description: String
}
#[derive(Debug,Deserialize,Serialize)]
#[allow(non_camel_case_types,non_snake_case)]
struct time {
    //Time is a variable that should not be used outside the CalendarEvent type, there are cleaner ways to handle such a type
//naming must be this way for the calendar event type to serialize properly, not worth writing more code just for naming standards
    dateTime: NaiveDateTime,
    timeZone: String
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct AuthRequest {
    pub(crate) code: String,
    state: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}
pub struct EnvironmentVariables {
    pub address: String,
    pub port: String,
}
#[derive(Debug)]
pub struct MxQuery {
    pub id: i32,
    pub mx_index: i32,
    pub owner: i32,
    pub date: NaiveDate,
    pub title: String,
    pub description: String
}

/*
    Below  are the constructors and methods of the above types
 */

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
        format!("{} {} {}",month, day, self.year()%1000)

    }
    fn date_to_short_string(&self) -> String {
        format!("{}/{}/{}",self.month(), self.day(), self.year()%1000)
    }
}

impl MorningExercise {
    //constructors
    pub fn new_with_date(id:i32,
                         owner: GoogleUser,
                         date: NaiveDate,
                         title: String,
                         description: String,
                         _editors: Option<Vec<GoogleUser>>)
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
               _editors: Option<GoogleUser>)
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

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub(crate) fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

