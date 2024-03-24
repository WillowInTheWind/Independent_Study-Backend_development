use axum::http::StatusCode;
use crate::handlers::GenericUser;
use crate::mxdate_algorithim;
use time::Date;

trait DateToString {
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
    id: u32,
    index:u16,
    date: Date,
    owner: GenericUser,
    title: String,
    description: String,
    editors: Vec<GenericUser>
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
            date: mxdate_algorithim::index_to_date(),
            owner,
            title ,
            description ,
            editors ,
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
            index: mxdate_algorithim::date_to_index(),
            owner,
            title ,
            description ,
            editors ,
        }
    }
}
trait MxService {
    async fn get_mx_by_id(&self) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_date(&self) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_index(&self) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_title(&self) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mx_by_owner(&self) -> Result<MorningExercise, (StatusCode, String)>;
    async fn get_mxs(&self) ->Result<Vec<MorningExercise>, (StatusCode, String)>;
    async fn create_mx(&self) -> StatusCode;
    async fn delete_mx_by_id(&self) -> StatusCode;
    async fn delete_mx_by_index(&self) -> StatusCode;
    async fn delete_mx_by_title(&self) -> StatusCode;
    async fn edit_mx(&self) ->  StatusCode;
}