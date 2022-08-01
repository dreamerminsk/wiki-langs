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
        let mut entries = fs::read_dir(path)?
            .and_then(|res| res.map(|e| Segment::open(e.path())))
            .collect::<Result<Vec<_>, io::Error>>()?;
        Segments { entries }
    }
}
