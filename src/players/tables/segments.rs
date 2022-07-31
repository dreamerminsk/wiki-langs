use std::{ffi::OsStr, iter::Iterator, path::Path};

pub struct Segments<'a> {
    root: &'a Path,
}

impl Segments<'a> {
    pub fn new<S: AsRef<OsStr> + ?Sized>(root: &S) -> Segments {
        Segments {
            root: Path::new(root),
        }
    }
}

impl Iterator for Segments<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        Some(())
    }
}
