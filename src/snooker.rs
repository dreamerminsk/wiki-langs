use crate::services::web;
use crate::services::web::entities::Link;
use crate::services::web::html::Extract;
use chrono::naive::MIN_DATE;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error::Error;
use std::hash::{Hash, Hasher};

pub const HOST: &str = "http://www.snooker.org";

pub const RESULTS: &str = "/res/index.asp?template=22&season=";
pub const UPCOMING_MATCHES: &str = "/res/index.asp?template=24";

pub const RANKINGS: &str = "/res/index.asp?template=31&season=";
pub const SEEDINGS: &str = "/res/index.asp?template=32&season=";
pub const POINTS: &str = "/res/index.asp?template=33&season=";

pub const PLAYER: &str = "/res/index.asp?player=";
pub const EVENT: &str = "/res/index.asp?event=";

pub fn results(season: usize) -> String {
    format!("{}{}{}", HOST, RESULTS, season)
}

pub fn upcoming_matches() -> String {
    format!("{}{}", HOST, UPCOMING_MATCHES)
}

pub fn rankings(season: usize) -> String {
    format!("{}{}{}", HOST, RANKINGS, season)
}

pub async fn get_player(snooker_id: usize) -> Result<Player, Box<dyn Error>> {
    let page = web::get(format!("{}{}{}", HOST, PLAYER, snooker_id)).await?;

    let info_text = page
        .extract_text("div.info")
        .unwrap_or_else(|| "".to_string());
    println!("info_text = ({:?})", info_text);

    let title = page.extract_text("title").unwrap_or_else(|| "".to_string());

    Ok(Player {
        full_name: extract_name(&title)
            .unwrap_or_else(|| extract_team(&title).unwrap_or_else(|| title.clone())),
        nation: extract_nation(&info_text).unwrap_or_default(),
        birthday: extract_date(&info_text).unwrap_or(MIN_DATE),
        snooker_id,
        cuetracker_id: None,
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
        static ref NATION_RE: Regex = Regex::new(r"Nationality:\s+?\((?P<nation>.*?)\);").unwrap();
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
        .filter(|l| l.url.contains("cuetracker")).map(|l|    _extract_ct_id(l.url.as_str())).first()
   
}

fn _extract_ct_id(text: &str) -> Option<String> {
    lazy_static! {
        static ref CT_RE: Regex = Regex::new(r"/Players/(?P<ctid>.*?)/.*?").unwrap();
    }
    CT_RE
        .captures(text)
        .and_then(|cap| cap.name("ctid").map(|ctid| ctid.as_str().to_string()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub full_name: String,
    pub nation: String,
    pub birthday: NaiveDate,
    pub snooker_id: usize,
    pub cuetracker_id: Option<String>,
    pub wikidata_id: Option<String>,
    pub wiki_id: Option<String>,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.birthday.cmp(&other.birthday)
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
        if value.url.starts_with(PLAYER) {
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
        if value.url.starts_with(EVENT) {
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
