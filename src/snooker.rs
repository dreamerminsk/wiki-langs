use crate::html::Link;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use url::Url;

pub const HOST: &str = "http://www.snooker.org";

pub const RESULTS: &str = "/res/index.asp?template=22&season={}";
pub const UPCOMING_MATCHES: &str = "/res/index.asp?template=24";

pub const RANKINGS: &str = "/res/index.asp?template=31&season={}";
pub const SEEDINGS: &str = "/res/index.asp?template=32&season={}";
pub const POINTS: &str = "/res/index.asp?template=33&season={}";

pub const PLAYER: &str = "/res/index.asp?player=";
pub const EVENT: &str = "/res/index.asp?event=";

fn query(u: Url) -> HashMap<String, String> {
    u.query_pairs().into_owned().collect()
}

#[derive(Debug)]
pub struct PlayerLink {
    snooker_id: u32,
    full_name: String,
}

impl Ord for PlayerLink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.snooker_id.cmp(&other.snooker_id)
    }
}

impl PartialOrd for PlayerLink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PlayerLink {
    fn eq(&self, other: &Self) -> bool {
        self.snooker_id == other.snooker_id
    }
}

impl Eq for PlayerLink {}

impl Hash for PlayerLink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.snooker_id.hash(state);
    }
}

impl TryFrom<&Link> for PlayerLink {
    type Error = &'static str;

    fn try_from(value: &Link) -> Result<Self, Self::Error> {
        if value.url.starts_with(PLAYER) {
            Ok(PlayerLink {
                snooker_id: extract_number(&value.url),
                full_name: value.title.clone(),
            })
        } else {
            Err("GreaterThanZero only accepts value superior than zero!")
        }
    }
}

#[derive(Debug)]
pub struct EventLink {
    snooker_id: u32,
    title: String,
}

impl Ord for EventLink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.snooker_id.cmp(&other.snooker_id)
    }
}

impl PartialOrd for EventLink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EventLink {
    fn eq(&self, other: &Self) -> bool {
        self.snooker_id == other.snooker_id
    }
}

impl Eq for EventLink {}

impl Hash for EventLink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.snooker_id.hash(state);
    }
}

impl TryFrom<Link> for EventLink {
    type Error = &'static str;

    fn try_from(value: Link) -> Result<Self, Self::Error> {
        if value.url.starts_with(EVENT) {
            Ok(EventLink {
                snooker_id: extract_number(value.url),
                title: value.title,
            })
        } else {
            Err("GreaterThanZero only accepts value superior than zero!")
        }
    }
}



fn extract_number(text: &String) -> u32 {
    text.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .unwrap_or(0)
}
