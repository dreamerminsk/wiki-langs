use chrono::NaiveDateTime;
use std::{
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
        let files = fs::read_dir("./players").ok()?;
        let mut rows: Vec<String> = files
            .into_iter()
            .filter_map(|di| di.ok())
            .map(|di| {
                format!(
                    "| {}s | [{}](players/{}) |",
                    self.osstr_to_year(di.file_name().as_os_str()),
                    di.metadata().unwrap().len(),
                    di.file_name().into_string().unwrap()
                )
            })
            .collect();
        rows.sort();
        Some(format!(
            "## players <sub>last modified: {}</sub>\r\n{}\r\n{}\r\n",
            Utc::now().to_rfc2822(),
            PLAYERS_HEADER,
            rows.join("\r\n")
        ))
    }

    fn osstr_to_year(&self, r#fn: &OsStr) -> String {
        r#fn.to_str().unwrap_or("0000").chars().take(4).collect()
    }
}
