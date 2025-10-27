use crate::app::web::{self, entities::Link, html::HtmlExt};
use crate::snooker::entities::Player;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    error::Error,
    hash::{Hash, Hasher},
};

pub mod urls;

pub mod entities;

pub async fn get_player(snooker_id: usize) -> Result<Player, Box<dyn Error>> {
    let page = web::get(format!("{}{}{}", urls::HOST, urls::PLAYER, snooker_id)).await?;

    let title = page.extract_text("title").unwrap_or_else(|| "".to_string());
    println!("title = ({:?})", title);

    let info_text = page.extract_text(".info").unwrap_or_else(|| "".to_string());
    println!("info_text = ({:?})", info_text);

    Ok(Player {
        full_name: extract_name(&title)
            .unwrap_or_else(|| extract_team(&title).unwrap_or_else(|| title.clone())),
        nation: extract_nation(&info_text).unwrap_or_default(),
        birthday: extract_date(&info_text),
        snooker_id,
        cuetracker_id: extract_ct_id(&page),
        wikidata_id: None,
        wiki_id: None,
    })
}

fn extract_name(input: &str) -> Option<String> {
    lazy_static! {
        static ref NAME_RE: Regex = Regex::new(r"(?P<name>.*?) - Players - snooker.org").unwrap();
    }
    NAME_RE
        .captures(input)
        .and_then(|cap| cap.name("name").map(|name| name.as_str().to_string()))
}

fn extract_team(input: &str) -> Option<String> {
    lazy_static! {
        static ref TEAM_RE: Regex = Regex::new(r"(?P<name>.*?) - Teams - snooker.org").unwrap();
    }
    TEAM_RE
        .captures(input)
        .and_then(|cap| cap.name("name").map(|name| name.as_str().to_string()))
}

fn extract_nation(input: &str) -> Option<String> {
    lazy_static! {
        static ref NATION_RE: Regex = Regex::new(r"\s*?\((?P<nation>.*?)\);").unwrap();
    }
    NATION_RE
        .captures(input)
        .and_then(|cap| cap.name("nation").map(|nation| nation.as_str().to_string()))
}

fn extract_date(text: &str) -> Option<NaiveDate> {
    lazy_static! {
        static ref DATE_RE: Regex =
            Regex::new(r"Born:\s+?(?P<date>\d{1,2}?\s+?[A-Za-z]{3}?\s+?\d{4})").unwrap();
    }
    DATE_RE.captures(text).and_then(|cap| {
        cap.name("date")
            .map(|d| d.as_str())
            .and_then(|s| NaiveDate::parse_from_str(s, "%e %b %Y").ok())
    })
}

fn extract_ct_id(page: &Html) -> Option<String> {
    page.extract_links()
        .into_iter()
        .filter(|l| l.url.contains("cuetracker"))
        .filter_map(|l| _extract_ct_id(l.url.as_str()))
        .next()
}

fn _extract_ct_id(text: &str) -> Option<String> {
    lazy_static! {
        static ref CT_RE: Regex = Regex::new(r"/Players/(?P<ctid>[^/]+)").unwrap();
    }
    CT_RE
        .captures(text)
        .and_then(|cap| cap.name("ctid").map(|ctid| ctid.as_str().to_string()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerLink {
    pub snooker_id: usize,
    pub full_name: String,
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
        if value.url.starts_with(urls::PLAYER) {
            Ok(PlayerLink {
                snooker_id: extract_first_number(&value.url),
                full_name: value.title.clone(),
            })
        } else {
            Err("GreaterThanZero only accepts value superior than zero!")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventLink {
    pub snooker_id: usize,
    pub title: String,
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

impl TryFrom<&Link> for EventLink {
    type Error = &'static str;

    fn try_from(value: &Link) -> Result<Self, Self::Error> {
        if value.url.starts_with(urls::EVENT) {
            Ok(EventLink {
                snooker_id: extract_first_number(&value.url),
                title: value.title.clone(),
            })
        } else {
            Err("GreaterThanZero only accepts value superior than zero!")
        }
    }
}

fn extract_number(text: &str) -> usize {
    text.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0)
}

fn extract_first_number(text: &str) -> usize {
    text.chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0)
}
