use std::iter::Iterator;

pub struct Segments {}

impl Iterator for Segments {
    type Item = ();

    pub fn next(&mut self) -> Option<Self::Item> {
        Some(())
    }
}
