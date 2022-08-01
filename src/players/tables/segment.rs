use crate::snooker::entities::Player;
use csv;
use std::{convert::AsRef, ffi::OsStr, io, path::Path};

pub struct Segment {
    items: Vec<Player>,
}

impl Segment {
    pub fn open<P: AsRef<Path> + AsRef<OsStr>>(path: P) -> io::Result<Segment> {
        let mut items = vec![];
        if Path::new(&path).exists() {
            let mut source_reader = csv::Reader::from_path(&path)?;
            for p in source_reader.deserialize() {
                let p: Player = p?;
                items.push(p);
            }
        }
        Ok(Segment { items })
    }
}





impl IntoIterator for Segment {
    type Item =     Player;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
