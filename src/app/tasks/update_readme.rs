use crate::players::tables::Segments;
use chrono::{Datelike, Utc};
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs::{self, OpenOptions},
    io::Write,
};

static README_PATH: &str = "./README.md";

static SHIELDS_PATH: &str = "./README/SHIELDS.md";

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

        let content = format!("## wiki-langs\r\n{}", self.content());

        file.write_all(content.as_bytes()).ok()?;
        file.flush().ok()?;
        Some(())
    }

    fn content(&self) -> String {
        format!(
            "{}\r\n{}\r\n",
            self.shields().unwrap_or_else(|| "".to_string()),
            self.players().unwrap_or_else(|| "".to_string())
        )
    }

    fn shields(&self) -> Option<String> {
        fs::read_to_string(SHIELDS_PATH).ok()
    }

    


    fn players(&self) -> Option<String> {
        let segs = Segments::open("./players").ok()?;
        let mut years = BTreeMap::<String, usize>::new();
        segs.into_iter().flat_map(|s| s.into_iter()).for_each(|p| {
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
            "## players\r\n<sup>last modified: {}</sup>\r\n{}\r\n{}\r\n",
            Utc::now().to_rfc2822(),
            PLAYERS_HEADER,
            rows.join("\r\n")
        ))
    }
}
