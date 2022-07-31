use std::{iter::Iterator, path::Path,ffi:OsStr};

pub struct Segments {
    root: Path,
}

impl Segments {
    pub fn new<S: AsRef<OsStr> + ?Sized>(root: &S) -> Segments {
        Segments {
            root: Path::new(root),
        }
    }
}

impl Iterator for Segments {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        Some(())
    }
}
