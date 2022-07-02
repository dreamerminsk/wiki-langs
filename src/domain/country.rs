use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub iso_num: Option<String>,
}
