use chrono::NaiveDate;

#[derive(Debug)]
pub struct Player {
    pub cuetracker_id: String,
pub full_name:String,
pub birthday:Option<NaiveDate>,
}
