use std::iter::Iterator;

pub struct Segments {}

impl Iterator for Segments {
    type Item = ();

 fn next(&mut self) -> Option<Self::Item> {
        Some(())
    }
}
