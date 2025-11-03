use crate::{players::tables::Segments, snooker::entities::Player};
use chrono::Datelike;
use std::io::Write;
use std::{collections::BTreeMap, fs::OpenOptions};

static NATION_PATH: &str = "./REPORTS/NATIONS.md";

pub struct NationStats {}

impl NationStats {
    pub fn new() -> Self {
        NationStats {}
    }

    pub fn execute(&self) -> Option<()> {
        let mut file = OpenOptions::new()
            .create(true)
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
        format!("{}\r\n", self.nations().unwrap_or_else(|| "".to_string()),)
    }

    fn nations(&self) -> Option<String> {
        let segs = Segments::open("./players").ok()?;
        let mut nation_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut decade_counts: BTreeMap<(String, i32), usize> = BTreeMap::new();
        let mut min_decade = 3000;
        let mut max_decade = 1000;

        segs.into_iter()
            .flat_map(|s| s.into_iter())
            .for_each(|player| {
                *nation_counts.entry(player.nation.clone()).or_insert(0) += 1;
                if let Some(birthday) = player.birthday {
                    let decade = (birthday.year() / 10) * 10;

                    if decade < min_decade {
                        min_decade = decade;
                    }
                    if decade > max_decade {
                        max_decade = decade;
                    }
                    *decade_counts
                        .entry((player.nation.clone(), decade))
                        .or_insert(0) += 1;
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

        let mut sorted_by_value: Vec<_> = nation_counts.iter().collect();
        sorted_by_value.sort_by(|a, b| b.1.cmp(&a.1));

        for (nation, total_count) in sorted_by_value.iter() {
            let mut row = format!("| {} ", nation);
            for decade in (min_decade..=max_decade).step_by(10) {
                let count = decade_counts
                    .get(&(nation.to_string(), decade))
                    .unwrap_or(&0);
                row.push_str(&format!("| {} ", count));
            }
            row.push_str(&format!("| {} |\n", total_count));
            table.push_str(&row);
        }

        Some(format!("## nations\r\n{}\r\n", table,))
    }
}
