use crate::wiki::entities::Page;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    convert::From,
    hash::{Hash, Hasher},
};

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

impl Country {
    pub fn wiki(&self, page: &Page) -> Self {
        Country {
            name: self.name.clone(),
            iso_num: self.iso_num.clone(),
            iso_2: self.iso_2.clone(),
            iso_3: self.iso_3.clone(),
            ioc_cc: self.ioc_cc.clone(),
            fifa_cc: self.fifa_cc.clone(),
            wiki_data_id: page.wikidata.clone(),
            wiki_id: Some(page.title.clone()),
        }
    }
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
        let mut some_pairs = 0;
        let mut eq_pairs = 0;

        if let (Some(this), Some(that)) = (self.iso_num.as_ref(), other.iso_num.as_ref()) {
            some_pairs += 1;
            if this == that {
                eq_pairs += 1;
            }
        }

        if self.iso_num.is_some() && other.iso_num.is_some() {
            some_pairs += 1;
            if self.iso_num == other.iso_num {
                eq_pairs += 1;
            }
        }

        if self.iso_2.is_some() && other.iso_2.is_some() {
            some_pairs += 1;
            if self.iso_2 == other.iso_2 {
                eq_pairs += 1;
            }
        }

        if self.iso_3.is_some() && other.iso_3.is_some() {
            some_pairs += 1;
            if self.iso_3 == other.iso_3 {
                eq_pairs += 1;
            }
        }

        if self.ioc_cc.is_some() && other.ioc_cc.is_some() {
            some_pairs += 1;
            if self.ioc_cc == other.ioc_cc {
                eq_pairs += 1;
            }
        }

        if self.fifa_cc.is_some() && other.fifa_cc.is_some() {
            some_pairs += 1;
            if self.fifa_cc == other.fifa_cc {
                eq_pairs += 1;
            }
        }

        if self.wiki_data_id.is_some() && other.wiki_data_id.is_some() {
            some_pairs += 1;
            if self.wiki_data_id == other.wiki_data_id {
                eq_pairs += 1;
            }
        }

        if self.wiki_id.is_some() && other.wiki_id.is_some() {
            some_pairs += 1;
            if self.wiki_id == other.wiki_id {
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
