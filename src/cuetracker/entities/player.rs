use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player{
    pub cuetracker_id: String,
    pub full_name: String,
    pub birthday: Option<NaiveDate>,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cuetracker_id
            .cmp(&other.cuetracker_id)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.cuetracker_id == other.cuetracker_id
    }
}

impl Eq for Player {}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cuetracker_id.hash(state);
    }
}
