use crate::{players::tables::Segments, snooker::entities::Player};
use chrono::{Datelike, Utc};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::Write,
};

static README_PATH: &str = "./README.md";

static SHIELDS_PATH: &str = "./README/SHIELDS.md";

static WORLDS_PATH: &str = "./REPORTS/TITLES–20.md";

static PLAYERS_HEADER: &str = "| births | players |\r\n| :----: | ------: |";

pub struct UpdateReadMe {}

impl UpdateReadMe {
    pub fn new() -> Self {
        UpdateReadMe {}
    }

    pub fn execute(&self) -> Option<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(README_PATH)
            .ok()?;

        let content = format!("{}", self.content());

        file.write_all(content.as_bytes()).ok()?;
        file.flush().ok()?;
        Some(())
    }

    fn content(&self) -> String {
        format!(
            "{}\r\n{}\r\n## wiki-langs\r\n{}\r\n",
            self.worlds().unwrap_or_else(|| "".to_string()),
            self.players().unwrap_or_else(|| "".to_string()),
            self.shields().unwrap_or_else(|| "".to_string())
        )
    }

    fn shields(&self) -> Option<String> {
        fs::read_to_string(SHIELDS_PATH).ok()
    }

    fn worlds(&self) -> Option<String> {
        fs::read_to_string(WORLDS_PATH).ok()
    }

    fn players(&self) -> Option<String> {
        let segs = Segments::open("./players").ok()?;
        let mut years = BTreeMap::<String, usize>::new();
        let mut births = BTreeSet::<Player>::new();
        let mut milles = BTreeSet::<Player>::new();
        let now = Utc::now();
        segs.into_iter().flat_map(|s| s.into_iter()).for_each(|p| {
            if p.birthday.is_some() {
                let bd = p.birthday.unwrap();
                if bd.month().eq(&now.month()) && bd.day().eq(&now.day()) {
                    births.insert(p.clone());
                }
                let d = now.naive_utc().date().signed_duration_since(bd);
                if (d.num_days() + 1) % 1000 == 0 {
                    milles.insert(p.clone());
                }
            }
            let y = p
                .birthday
                .map(|bd| (10 * (bd.year() / 10)).to_string())
                .unwrap_or_else(|| "0000".to_string());
            years.entry(y).and_modify(|e| *e += 1).or_insert(1);
        });
        let mut rows: Vec<String> = years
            .iter()
            .map(|(k, v)| format!("| {}s | {} |", k, v))
            .collect();
        rows.sort();

        Some(format!(
            "## players\r\n{}\r\n{}\r\n\r\n{}\r\n{}\r\n",
            PLAYERS_HEADER,
            rows.join("\r\n"),
            self.births(&births),
            self.milles(&milles)
        ))
    }

    fn births(&self, players: &BTreeSet<Player>) -> String {
        let mut brows: Vec<String> = players.iter().map(|v| self.birth_row(&v)).collect();
        brows.sort();
        format!(
            "#### ***born on {}***\r\n{}\r\n",
            Utc::now().format("%B %e"),
            brows.join("\r\n")
        )
    }

    fn birth_row(&self, player: &Player) -> String {
        let mut links = format!(
            "[Snooker](http://www.snooker.org/res/index.asp?player={})",
            player.snooker_id
        );
        if player.cuetracker_id.is_some() {
            links = format!(
                "{}, [CueTracker](http://cuetracker.net/Players/{}/)",
                links,
                player.cuetracker_id.as_ref().unwrap()
            );
        }
        format!(
            "{}, {}, {} <sub><sup>{}</sup></sub>\r\n",
            player.birthday.unwrap().year(),
            player.full_name,
            Utc::now().year() - player.birthday.unwrap().year(),
            links,
        )
    }

    fn milles(&self, players: &BTreeSet<Player>) -> String {
        let mut mrows: Vec<String> = players.iter().map(|v| self.mille_row(&v)).collect();
        mrows.sort();
        format!(
            "#### ***milleversary on {}***\r\n{}\r\n",
            Utc::now().format("%B %e, %Y"),
            mrows.join("\r\n")
        )
    }

    fn mille_row(&self, player: &Player) -> String {
        let mut links = format!(
            "[Snooker](http://www.snooker.org/res/index.asp?player={})",
            player.snooker_id
        );
        if player.cuetracker_id.is_some() {
            links = format!(
                "{}, [CueTracker](http://cuetracker.net/Players/{}/)",
                links,
                player.cuetracker_id.as_ref().unwrap()
            );
        }
        format!(
            "{}, {}, {} <sub><sup>{}</sup></sub>\r\n",
            player.birthday.unwrap().year(),
            player.full_name,
            Utc::now()
                .naive_utc()
                .date()
                .signed_duration_since(player.birthday.unwrap())
                .num_days()
                + 1,
            links,
        )
    }
}
