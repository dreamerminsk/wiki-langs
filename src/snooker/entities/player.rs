use chrono::naive::MIN_DATE;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error::Error;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub full_name: String,
    pub nation: String,
    pub birthday: Option<NaiveDate>,
    pub snooker_id: usize,
    pub cuetracker_id: Option<String>,
    pub wikidata_id: Option<String>,
    pub wiki_id: Option<String>,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.birthday
            .unwrap_or(MIN_DATE)
            .cmp(&other.birthday.unwrap_or(MIN_DATE))
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.snooker_id == other.snooker_id
    }
}

impl Eq for Player {}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.snooker_id.hash(state);
    }
}
