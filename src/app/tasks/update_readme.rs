use std::error::Error;
use std::fs::{OpenOptions,File};

pub struct UpdateReadMe {}

impl UpdateReadMe {
    fn new() -> Self {
        UpdateReadMe {}
    }

    fn execute(self) -> Result<(), Box<dyn Error>> {
        let shields = fs::read_to_string("./README/SHIELDS.md")?;
   let mut file = OpenOptions::new().read(true).write(true).open()?;
        Ok(())
    }
}
