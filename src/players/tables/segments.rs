use crate::players::tables::Segment;
use std::{
    fs::{self, ReadDir,DirEntry},
    io,
    iter::Iterator,
    path::Path,
};

pub struct Segments {
    inner: ReadDir,
}

impl Segments {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Segments> {
        fs::read_dir(path).map(|rd| Segments { inner: rd })
    }
}

impl Iterator for Segments {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next())
    }
}
