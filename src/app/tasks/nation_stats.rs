use crate::{players::tables::Segments, snooker::entities::Player};
use chrono::{Datelike, Utc};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs::{self, OpenOptions},
    io::Write,
};

static NATION_PATH: &str = "./REPORTS/NATIONS.md";

pub struct NationStats {}

impl NationStats {
    pub fn new() -> Self {
        NationStats {}
    }

    pub fn execute(&self) -> Option<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(NATION_PATH)
            .ok()?;

        let content = format!("{}", self.content());

        file.write_all(content.as_bytes()).ok()?;
        file.flush().ok()?;
        Some(())
    }

    fn content(&self) -> String {
        format!(
            "## player by nations\r\n{}\r\n",
            self.nations().unwrap_or_else(|| "".to_string()),
        )
    }

    fn nations(&self) -> Option<String> {
        let segs = Segments::open("./players").ok()?;
        let mut counts: BTrerMap<String, usize> = BTreeMap::new();
        segs.into_iter().flat_map(|s| s.into_iter()).for_each(|p| {
       *counts.entry(p.nation).or_insert(0) += 1;
            if p.birthday.is_some() {
                let bd = p.birthday.unwrap();
                *counts.entry(bd.year()).or_insert(0) += 1;
            }
        });

        let min_year = counts.keys().min().unwrap_or(&1900).clone();
        let max_year = counts.keys().max().unwrap_or(&2020).clone();
        let min_decade = min_year - min_year % 10;
        let max_decade = max_year - max_year % 10;

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

        Some(format!("## players\r\n{}\r\n", table,))
    }
}
