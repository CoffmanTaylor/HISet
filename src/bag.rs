//! A bag that is both indexable and hash-able. The index order is sorted ord of all items in the bag.

use std::{hash::Hash, iter::FromIterator, ops::Index};

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct HIBag<T> {
    items: Vec<T>,
}

impl<T> HIBag<T> {
    pub fn new() -> HIBag<T> {
        HIBag { items: Vec::new() }
    }

    /// Will insert the given item into the bag.
    pub fn insert(&mut self, item: T)
    where
        T: Ord,
    {
        match self.items.binary_search(&item) {
            Ok(i) | Err(i) => self.items.insert(i, item),
        }
    }

    /// Removes the item from the bag if present.
    pub fn remove(&mut self, item: &T) -> Option<T>
    where
        T: Ord,
    {
        match self.items.binary_search(item) {
            Ok(i) => Some(self.items.remove(i)),
            Err(_) => None,
        }
    }

    /// Returns true iff the bag contains the given item.
    pub fn contains(&self, item: &T) -> bool
    where
        T: Ord,
    {
        self.items.binary_search(item).is_ok()
    }

    /// Returns the size of the bag.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns an Iterator over references of the items stored. Iteration order is the order from Ord.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    /// Remove the item at the given index. Panics if the index is out of bounds.
    pub fn remove_index(&mut self, index: usize) -> T {
        assert!(index < self.items.len(), "Index out of bounds");

        self.items.remove(index)
    }

    /// Returns a reference to the item at the given index. Panics if the index is out of bounds.
    pub fn get_index(&self, index: usize) -> &T {
        assert!(index < self.items.len(), "Index out of bounds");

        &self.items[index]
    }
}

impl<T> Index<usize> for HIBag<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> IntoIterator for HIBag<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> FromIterator<T> for HIBag<T>
where
    T: Ord,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = HIBag::new();

        iter.into_iter().for_each(|t| {
            set.insert(t);
            ()
        });

        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_1() {
        let mut bag = HIBag::new();

        bag.insert(1);

        assert_eq!(1, bag[0]);
    }

    #[test]
    fn can_contain_duplicates() {
        let mut bag = HIBag::new();

        bag.insert(1);
        bag.insert(1);

        assert_eq!(2, bag.len());
        assert_eq!(1, bag[0]);
        assert_eq!(1, bag[1]);
    }

    #[test]
    fn iteration_order_is_ord() {
        let mut bag = HIBag::new();

        bag.insert(3);
        bag.insert(1);
        bag.insert(2);

        let mut iter = bag.iter();

        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn can_contain_different_values() {
        let mut bag = HIBag::new();

        bag.insert(1);
        bag.insert(2);

        assert_eq!(2, bag.len());
        assert_eq!(true, bag.contains(&1));
        assert_eq!(true, bag.contains(&2));
    }
    #[test]
    fn can_remove_items() {
        let mut bag = HIBag::new();

        bag.insert(1);
        bag.insert(2);
        bag.remove(&1);

        assert_eq!(1, bag.len());
        assert_eq!(false, bag.contains(&1));
    }
}
