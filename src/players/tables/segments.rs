use crate::players::tables::Segment;
use std::{
    fs::{self, DirEntry, ReadDir},
    io,
    iter::Iterator,
    path::Path,
};

pub struct Segments {
    entries: Vec<Segment>,
}

impl Segments {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Segments> {
        let entries = fs::read_dir(path)?
            .map(|res| res.and_then(|e| Segment::open(e.path())))
            .collect::<Result<Vec<_>, io::Error>>()?;
        Ok(Segments { entries })
    }
}

impl IntoIterator for Segments {
    type Item = Segment;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
