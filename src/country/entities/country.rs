use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::From;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub iso_num: Option<String>,
    pub iso_2: Option<String>,
    pub iso_3: Option<String>,
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

impl From<String> for Country {
    fn from(value: String) -> Self {
        Country {
            name: value,
            iso_num: None,
            iso_2: None,
            iso_3: None,
            ioc_cc: None,
            fifa_cc: None,
            wiki_data_id: None,
            wiki_id: None,
        }
    }
}
