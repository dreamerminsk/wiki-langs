use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub iso_num: Option<String>,
    pub iso_alpha_2: Option<String>,
    pub iso_alpha_3: Option<String>,
    pub ioc_cc: Option<String>,
    pub fifa_cc: Option<String>,
    pub wiki_data_id: Option<String>,
    pub wiki_id: Option<String>,
}

impl Ord for Country {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Country {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Country {}

impl Hash for Country {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}







impl       From<String>    for   Country   {
fn from(value:String)->Self{
Country {
    name: value.to_owned(),
    iso_num: None,
    iso_alpha_2: None,
   iso_alpha_3: None,
    ioc_cc: None,
   fifa_cc: None,
   wiki_data_id: None,
    wiki_id: None,
}
}  
}
