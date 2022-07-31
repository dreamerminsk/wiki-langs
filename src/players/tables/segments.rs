use std::{ffi::OsStr, iter::Iterator, path::PathBuf};
use crate::players::tables::Segment;

pub struct Segments {
    root: PathBuf,
}

impl Segments {
    pub fn new(path: String) -> Segments {
        Segments {
            root: PathBuf::from(path),
        }
    }
}

impl Iterator for Segments {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Segment {})
    }
}
