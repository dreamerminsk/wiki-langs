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
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut decades = BTreeMap::new();
        let mut min_decade = 3000;
        let mut max_decade = 1000;

        segs.into_iter().flat_map(|s| s.into_iter()).for_each(|p| {
            *counts.entry(p.nation).or_insert(0) += 1;
            if p.birthday.is_some() {
                let bd = p.birthday.unwrap();
                let decade = bd.year() - bd.year() % 10;
                if decade < min_decade {
                    min_decade = decade;
                }
                if decade > max_decade {
                    max_decade = decade;
                }
                *decades.entry((p.nation, decade)).or_insert(0) += 1;
            }
        });

        let mut header = String::from("| Nation ");
        for decade in (min_decade..=max_decade).step_by(10) {
            header.push_str(&format!("| {} ", decade));
        }
        header.push_str("| Total |\n");

        let mut separator = String::from("|:-------");
        for _ in (min_decade..=max_decade).step_by(10) {
            separator.push_str("|:-------:");
        }
        separator.push_str("|:-------------:|\n");

        let mut table = header;
        table.push_str(&separator);

        let mut sorted_by_value: Vec<(&str, usize)> = map.into_iter().collect();
        sorted_by_value.sort_by(|a, b| b.1.cmp(&a.1));

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
