use std::{ffi::OsStr, iter::Iterator, path::PathBuf};

pub struct Segments {
    root: PathBuf,
}

impl Segments {
    pub fn new<S: AsRef<OsStr>>(path: S) -> Segments {
        Segments {
            root: PathBuf::from(path),
        }
    }
}

impl Iterator for Segments {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        Some(())
    }
}
