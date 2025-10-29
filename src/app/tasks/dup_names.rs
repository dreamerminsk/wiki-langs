use crate::{players::tables::Segments, snooker::entities::Player};
use chrono::{Datelike, Utc};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs::{self, OpenOptions},
    io::Write,
};

static WORLDS_PATH: &str = "./REPORTS/TITLES–20.md";

pub struct DupNames {}

impl DupNames {
    pub fn new() -> Self {
        DupNames {}
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
            "{}\r\n",
            self.players().unwrap_or_else(|| "".to_string()),
        )
    }

    

    

    fn players(&self) -> Option<String> {
        let segs = Segments::open("./players").ok()?;
        
        segs.into_iter().flat_map(|s| s.into_iter()).for_each(|p| {
            if p.birthday.is_some() {
                
            }
        });

        let mut header = String::from("| Decade ");
        for year in 0..10 {
            header.push_str(&format!("| Year +{} ", year));
        }
        header.push_str("| Decade Total |\n");

        let mut separator = String::from("|:-------");
        for _ in 0..10 {
            separator.push_str("|:-------:");
        }
        separator.push_str("|:-------------:|\n");

        let mut table = header;
        table.push_str(&separator);

        for decade in (min_decade..=max_decade).step_by(10) {
            let mut row = format!("| {}s ", decade);
            let mut decade_total = 0;
            for year in decade..decade + 10 {
                let count = counts.get(&year).unwrap_or(&0);
                decade_total += count;
                row.push_str(&format!("| {} ", count));
            }
            row.push_str(&format!("| {} |\n", decade_total));
            table.push_str(&row);
        }

        Some(format!(
            "{}\r\n",
            table,
        ))
    }
}
