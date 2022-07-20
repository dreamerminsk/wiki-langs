use std::error::Error;
use std::fs::{File, OpenOptions};

static README_PATH: &str = "./README.md";

static SHIELDS_PATH: &str = "./README/SHIELDS.md";

static README_TEMPLATE: &str = "==wiki-langs\r\n{}\r\n";

static PLAYERS_TEMPLATE: &str =
    "===players\r\n| births | players |\r\n| :----: | ------: |\r\n{}\r\n";

pub struct UpdateReadMe {}

impl UpdateReadMe {
    pub fn new() -> Self {
        UpdateReadMe {}
    }

    pub fn execute(self) -> Option<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(README_PATH)?;

        let content = format!(README_TEMPLATE, self.shields());

        file.write_all(content.as_bytes())?;
        Some(())
    }

    fn shields(self) -> Option<String> {
        fs::read_to_string(SHIELDS_PATH)?
    }

    fn players(self) -> Option<String> {
        let files = fs::read_dir("./players")?;
        Some("")
    }
}
