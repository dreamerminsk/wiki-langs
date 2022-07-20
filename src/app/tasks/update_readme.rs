use std::error::Error;
use std::fs::{File, OpenOptions};

static README_PATH: &str = "./README.md";

pub struct UpdateReadMe {}

impl UpdateReadMe {
    fn new() -> Self {
        UpdateReadMe {}
    }

    fn execute(self) -> Result<(), Box<dyn Error>> {
        let shields = fs::read_to_string("./README/SHIELDS.md")?;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(README_PATH)?;
        file.write_all(shields.as_bytes())?;
        Ok(())
    }
}
