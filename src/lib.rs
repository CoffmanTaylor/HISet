//! A set that is both indexable and hashable. The index order is sorted ord of all items in the set.

use std::{
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct HISet<T> {
    items: Vec<T>,
}

impl<T> HISet<T> {
    pub fn new() -> HISet<T> {
        HISet { items: Vec::new() }
    }

    /// Will insert the given item into the set. Returns `false` if the item was not already in the set.
    pub fn insert(&mut self, item: T) -> bool
    where
        T: Ord,
    {
        match self.items.binary_search(&item) {
            Ok(_) => true,
            Err(i) => {
                self.items.insert(i, item);
                false
            }
        }
    }

    /// Removes the item from the set if present.
    pub fn remove(&mut self, item: &T) -> Option<T>
    where
        T: Ord,
    {
        match self.items.binary_search(item) {
            Ok(i) => Some(self.items.remove(i)),
            Err(_) => None,
        }
    }

    /// Returns true iff the set contains the given item.
    pub fn contains(&self, item: &T) -> bool
    where
        T: Ord,
    {
        self.items.binary_search(item).is_ok()
    }

    /// Returns the size of the set.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn remove_index(&mut self, index: usize) -> T {
        assert!(index < self.items.len(), "Index out of bounds");

        self.items.remove(index)
    }

    pub fn get_index(&self, index: usize) -> &T {
        assert!(index < self.items.len(), "Index out of bounds");

        &self.items[index]
    }

    pub fn get_index_mut(&mut self, index: usize) -> &T {
        assert!(index < self.items.len(), "Index out of bounds");

        &mut self.items[index]
    }
}

impl<T> Index<usize> for HISet<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> IndexMut<usize> for HISet<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl<T> IntoIterator for HISet<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
