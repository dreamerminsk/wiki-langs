use crate::players::tables::Segment;
use std::{
    fs::{self, DirEntry, ReadDir},
    io,
    iter::Iterator,
    path::Path,
};

pub struct Segments {
    inner: ReadDir,
}

impl Segments {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Segments> {
        let read_dir    =           fs::read_dir(path)?;
let paths=read_dir.into_iter().map(|rd|rd.path()).collect();
    }
}

impl Iterator for Segments {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
