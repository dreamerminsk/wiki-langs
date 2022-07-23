use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write;

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
            .open(README_PATH)
            .ok()?;

        let content = format!("## wiki-langs\r\n{}\r\n", self.content());

        file.write_all(content.as_bytes()).ok()?;
        file.flush().ok()?;
        Some(())
    }

    fn content(&self) -> String {
        format!(
            "{}\r\n{}",
            self.shields().unwrap_or_else(|| "".to_string()),
            self.players().unwrap_or_else(|| "".to_string())
        )
    }

    fn shields(&self) -> Option<String> {
        fs::read_to_string(SHIELDS_PATH).ok()
    }

    fn players(&self) -> Option<String> {
        let files = fs::read_dir("./players").ok()?;
        let mut rows: Vec<String> = files
            .into_iter()
            .filter_map(|di| di.ok())
            .map(|di| {
                format!(
                    "| {}s | [{}](players/{}) |",
                    self.osstring_to_year(&di.file_name()),
                    di.metadata().unwrap().len(),
                    di.file_name().into_string().unwrap()
                )
            })
            .collect();
        rows.sort();
        Some(format!(
            "### players\r\n{}\r\n{}\r\n",
            PLAYERS_HEADER,
            rows.join("\r\n")
        ))
    }

    fn osstring_to_year(&self, r#fn: &OsString) -> String {
        r#fn.into_string()
            .unwrap_or("0000".to_string())
            .chars()
            .take(4)
            .collect()
    }
}
