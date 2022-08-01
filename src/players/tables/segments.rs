use crate::players::tables::Segment;
use std::{
    fs::{self, DirEntry, ReadDir},
    io,
    iter::Iterator,
    path::Path,
};

pub struct Segments {
    entries: Vec<&Path>,
}

impl Segments {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Segments> {
        let mut entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();
        Segments { entries }
    }
}

impl Iterator for Segments {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
