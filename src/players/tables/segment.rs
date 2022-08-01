use crate::snooker::entities::Player;
use csv;
use std::{io, path::Path};

pub struct Segment {
    items: Vec<Player>,
}

impl Segment {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Segment> {
        let mut items = vec![];
        if Path::new(path).exists() {
            let mut source_reader = csv::Reader::from_path(path)?;
            for p in source_reader.deserialize() {
                let p: Player = p?;
                items.push(&p);
            }
        }
        Ok(Segment { items })
    }
}
