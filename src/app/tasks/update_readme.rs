use chrono::Utc;
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
            self.players2().unwrap_or_else(|| "".to_string())
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
            "## players\r\n<sup>last modified: {}</sup>\r\n{}\r\n{}\r\n",
            Utc::now().to_rfc2822(),
            PLAYERS_HEADER,
            rows.join("\r\n")
        ))
    }

    fn osstr_to_year(&self, r#fn: &OsStr) -> String {
        r#fn.to_str().unwrap_or("0000").chars().take(4).collect()
    }

    fn players2(&self) -> Option<String> {
        let segs = Segments::open("./players").ok()?;
        let mut years = BTreeMap::<&str, uint>::new();
        segs.into_iter().flat_map(|s| s.into_iter()).for_each(|p| {
            let y = p
                .birthday
                .map(|bd| bd.year().to_string())
                .unwrap_or_else(|| "0000".to_string());
            years.entry(y.as_str()).and_modify(|e| *e += 1).or_insert(1);
        });
        let mut rows: Vec<String> = years
            .iter()
            .map(|k, v| format!("| {}s | {} |", k, v))
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
