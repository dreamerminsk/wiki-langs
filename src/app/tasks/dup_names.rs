use crate::{players::tables::Segments, snooker::entities::Player};
use chrono::{Datelike, Utc};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs::{self, OpenOptions},
    io::Write,
};

static DUP_PATH: &str = "./REPORTS/DUP–NAMES.md";

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
            .open(DUP_PATH)
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
    let mut name_map: HashMap<String, Vec<Player>> = HashMap::new();


    
    
}
        
        segs.into_iter().flat_map(|s| s.into_iter()).for_each(|p| {
            name_map.entry(p.name.clone()).or_default().push(p);
        });

for (name, group) in name_map {
        if group.len() > 1 {
            let row = format!("| {} | {} |\n", name, group);
table.push_str(&row);
        }
    }

      

        
        Some(format!(
            "{}\r\n",
            table,
        ))
    }
}
