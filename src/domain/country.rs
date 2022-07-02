use serde::{Deserialize, Serialize};

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
