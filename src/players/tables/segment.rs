use std::{io, path::Path};

pub struct Segment {}

impl Segment {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Segment> {
        Ok(Segment {})
    }
}
