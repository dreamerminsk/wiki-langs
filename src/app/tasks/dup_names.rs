use crate::{players::tables::Segments, snooker::entities::Player};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

static DUP_PATH: &str = "./REPORTS/DUP–NAMES.md";

pub struct DupNames {}

impl DupNames {
    pub fn new() -> Self {
        DupNames {}
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
        let mut name_map: HashMap<String, Vec<Player>> = HashMap::new();

        for player in segs.into_iter().flat_map(|s| s.into_iter()) {
            name_map
                .entry(player.name.clone())
                .or_default()
                .push(player);
        }

        let mut table = String::new();

        table.push_str("| Name | Count |\n");
        table.push_str("|------|-------|\n"); // Добавляем разделитель

        for (name, group) in name_map {
            if group.len() > 1 {
                let row = format!("| {} | {} |\n", name, group.len());
                table.push_str(&row);
            }
        }

        Some(table)
    }
}
