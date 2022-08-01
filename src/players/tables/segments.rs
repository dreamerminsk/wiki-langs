use crate::players::tables::Segment;
use std::{ffi::OsStr, fs::{self,ReadDir}, io, iter::Iterator, path::Path};

pub struct Segments {
    inner: ReadDir,
}

impl Segments {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Segments> {
        fs::read_dir(path).map(|rd| Segments { inner: rd })
    }
}

impl Iterator for Segments {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Segment {})
    }
}
