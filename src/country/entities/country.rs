use crate::wiki::entities::Page;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    convert::AsRef,
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

        if let (Some(this), Some(that)) = (self.iso_2.as_ref(), other.iso_2.as_ref()) {
            some_pairs += 1;
            if this == that {
                eq_pairs += 1;
            }
        }

        if let (Some(this), Some(that)) = (self.iso_3.as_ref(), other.iso_3.as_ref()) {
            some_pairs += 1;
            if this == that {
                eq_pairs += 1;
            }
        }

        if let (Some(this), Some(that)) = (self.ioc_cc.as_ref(), other.ioc_cc.as_ref()) {
            some_pairs += 1;
            if this == that {
                eq_pairs += 1;
            }
        }

        if let (Some(this), Some(that)) = (self.fifa_cc.as_ref(), other.fifa_cc.as_ref()) {
            some_pairs += 1;
            if this == that {
                eq_pairs += 1;
            }
        }

        if let (Some(this), Some(that)) = (self.wiki_data_id.as_ref(), other.wiki_data_id.as_ref())
        {
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

impl Eq for Country {}

impl Hash for Country {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<T: AsRef<str>> From<T> for Country {
    fn from<T: AsRef<str>>(value: T) -> Self {
        Country {
            name: String::from(value.as_ref()),
            iso_num: None,
            iso_2: None,
            iso_3: None,
            ioc_cc: None,
            fifa_cc: None,
            wiki_data_id: None,
            wiki_id: Some(String::from(value.as_ref())),
        }
    }
}
