use std::error::Error;
use std::fs::{File, OpenOptions};

static README_PATH: &str = "./README.md";

static SHIELDS_PATH: &str = "./README/SHIELDS.md";

static README_TEMPLATE: &str = "==wiki-langs\r\n{}\r\n";

pub struct UpdateReadMe {}

impl UpdateReadMe {
    fn new() -> Self {
        UpdateReadMe {}
    }

    fn execute(self) -> Result<(), Box<dyn Error>> {
        let shields = fs::read_to_string(SHIELDS_PATH)?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(README_PATH)?;

        let content = format!(README_TEMPLATE, shields);

        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
