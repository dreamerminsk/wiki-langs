use crate::players::tables::Segment;
use std::{ffi::OsStr, iter::Iterator, path::PathBuf};

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
