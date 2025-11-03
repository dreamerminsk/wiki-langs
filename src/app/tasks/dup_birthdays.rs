use crate::{players::tables::Segments, snooker::entities::Player};
use chrono::NaiveDate;
use std::collections::BTreeMap;
use std::fs::OpenOptions;

static DUP_PATH: &str = "./REPORTS/DUP–BIRTHDAYS.md";

pub struct DupBirthdays {}

impl DupBirthdays {
    pub fn new() -> Self {
        DupBirthdays {}
    }

    pub fn execute(&self) -> Option<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(DUP_PATH)
            .ok()?;

        let content = self.content().unwrap_or_else(|| String::from(""));

        file.write_all(content.as_bytes()).ok()?;
        file.flush().ok()?;
        Some(())
    }

    fn content(&self) -> Option<String> {
        let players_info = self.players()?;
        Some(format!("{}\r\n", players_info))
    }

    fn players(&self) -> Option<String> {
        let segs = Segments::open("./players").ok()?;
        let mut name_map: BTreeMap<NaiveDate, Vec<Player>> = BTreeMap::new();

        for player in segs.into_iter().flat_map(|s| s.into_iter()) {
            if let Some(birthday) = player.birthday {
                name_map.entry(birthday.clone()).or_default().push(player);
            }
        }

        let mut table = String::new();

        table.push_str("| Birthday | Count |\n");
        table.push_str("|------|-------|\n"); // Добавляем разделитель

        for (name, group) in name_map {
            if group.len() > 1 {
                let ids: Vec<_> = group.iter().map(|p| p.snooker_id).collect();
                let row = format!(
                    "| {} <br> {:?} | {} |\n",
                    name.format("%Y-%m-%d").to_string(),
                    ids,
                    group.len()
                );
                table.push_str(&row);
            }
        }

        Some(table)
    }
}
