use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        if let (Some(this_birthday), Some(that_birthday)) =
            (self.birthday.as_ref(), other.birthday.as_ref())
        {
            let birthday_order = this_birthday.cmp(that_birthday);
            if birthday_order != Ordering::Equal {
                return birthday_order;
            }
        }

        if let (Some(this_id), Some(that_id)) =
            (self.cuetracker_id.as_ref(), other.cuetracker_id.as_ref())
        {
            let id_order = this_id.cmp(that_id);
            if id_order != Ordering::Equal {
                return id_order;
            }
        }

        self.snooker_id.cmp(&other.snooker_id)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        let mut some_pairs = 0;
        let mut eq_pairs = 0;

        some_pairs += 1;
        if self.snooker_id == other.snooker_id {
            eq_pairs += 1;
        }

        if let (Some(this), Some(that)) =
            (self.cuetracker_id.as_ref(), other.cuetracker_id.as_ref())
        {
            some_pairs += 1;
            if this == that {
                eq_pairs += 1;
            }
        }

        if let (Some(this), Some(that)) = (self.wikidata_id.as_ref(), other.wikidata_id.as_ref()) {
            some_pairs += 1;
            if this == that {
                eq_pairs += 1;
            }
        }

        if let (Some(this), Some(that)) = (self.wiki_id.as_ref(), other.wiki_id.as_ref()) {
            some_pairs += 1;
            if this == that {
                eq_pairs += 1;
            }
        }

        if some_pairs > 0 {
            some_pairs == eq_pairs
        } else {
            false
        }
    }
}

impl Eq for Player {}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.snooker_id.hash(state);
    }
}
