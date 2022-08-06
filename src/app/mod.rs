pub mod tasks;

pub mod web;

pub trait Table {
    type Item;

    fn insert(item: Self::Item) -> bool;

    fn remove(item: Self::Item) -> bool;
}
