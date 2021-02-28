use std::borrow::Borrow;

use crate::vec::{Entry, OccupiedEntry, VacantEntry};

/// A trait extension that allows vectors to be treated as association lists.
pub trait AssocListExt<K, V> {
    /// Get a key's entry for in-place manipulation.
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut count = Vec::new();
    /// for x in vec!["a", "b", "c", "b"] {
    ///     *count.entry(x).or_insert(0) += 1;
    /// }
    /// assert_eq!(count.get(&"b"), Some(&2));
    /// ```
    fn entry(&mut self, key: K) -> Entry<K, V>;

    /// Get a reference to the value associated with a key.
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let map = vec![("a", 1), ("b", 2)];
    /// assert_eq!(map.get(&"a"), Some(&1));
    /// ```
    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized;

    /// Get a mutable reference to the value associated with a key.
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut map = vec![("a", 1), ("b", 2)];
    /// *map.get_mut(&"a").unwrap() += 1;
    /// assert_eq!(map.get(&"a"), Some(&2));
    /// ```
    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized;

    /// Insert a key-value pair into the association list.
    /// If the map previously had the key, then the old value is returned. Otherwise, `None` is
    /// returned.
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut map = vec![("b", 3)];
    /// assert_eq!(AssocListExt::insert(&mut map, "a", 1), None);
    /// assert_eq!(AssocListExt::insert(&mut map, "a", 2), Some(1));
    /// ```
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /// Remove a key from the map, returning the value if it was previously in the map.
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut map = vec![("a", 1)];
    /// assert_eq!(AssocListExt::remove(&mut map, "a"), Some(1));
    /// assert_eq!(AssocListExt::remove(&mut map, "a"), None);
    /// ```
    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized;
}

impl<K, V> AssocListExt<K, V> for Vec<(K, V)>
where
    K: PartialEq,
{
    fn entry(&mut self, key: K) -> Entry<K, V> {
        let found = self.iter_mut().enumerate().find(|(_, (k, _))| k == &key);
        match found {
            None => Entry::Vacant(VacantEntry::new(self, key)),
            Some((index, _)) => Entry::Occupied(OccupiedEntry::new(self, key, index)),
        }
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        self.iter().find(|(k, _)| k.borrow() == key).map(|(_, v)| v)
    }

    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        self.iter_mut()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.entry(key) {
            Entry::Occupied(mut entry) => Some(entry.insert(value)),
            Entry::Vacant(entry) => {
                entry.insert(value);
                None
            }
        }
    }

    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        let found = self
            .iter_mut()
            .enumerate()
            .find(|(_, (k, _))| k.borrow() == key);
        match found {
            None => None,
            Some((index, _)) => {
                let (_, v) = self.swap_remove(index);
                Some(v)
            }
        }
    }
}

/// This has the same API as [`AssocListExt`] but with the additional constraint `K: Eq`.
///
/// ```compile_fail
/// use assoc::AssocListStrictExt;
///
/// let map = vec![(1.0, 1), (2.0, 2)];
/// map.entry(1.0);
/// ```
pub trait AssocListStrictExt<K, V> {
    /// Get a key's entry for in-place manipulation.
    ///
    /// ```rust
    /// use assoc::AssocListStrictExt;
    ///
    /// let mut count = Vec::new();
    /// for x in vec!["a", "b", "c", "b"] {
    ///     *count.entry(x).or_insert(0) += 1;
    /// }
    /// assert_eq!(count.get(&"b"), Some(&2));
    /// ```
    fn entry(&mut self, key: K) -> Entry<K, V>;

    /// Get a reference to the value associated with a key.
    ///
    /// ```rust
    /// use assoc::AssocListStrictExt;
    ///
    /// let map = vec![("a", 1), ("b", 2)];
    /// assert_eq!(map.get(&"a"), Some(&1));
    /// ```
    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized;

    /// Get a mutable reference to the value associated with a key.
    ///
    /// ```rust
    /// use assoc::AssocListStrictExt;
    ///
    /// let mut map = vec![("a", 1), ("b", 2)];
    /// *map.get_mut(&"a").unwrap() += 1;
    /// assert_eq!(map.get(&"a"), Some(&2));
    /// ```
    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized;

    /// Insert a key-value pair into the association list.
    /// If the map previously had the key, then the old value is returned. Otherwise, `None` is
    /// returned.
    ///
    /// ```rust
    /// use assoc::AssocListStrictExt;
    ///
    /// let mut map = vec![("b", 3)];
    /// assert_eq!(AssocListStrictExt::insert(&mut map, "a", 1), None);
    /// assert_eq!(AssocListStrictExt::insert(&mut map, "a", 2), Some(1));
    /// ```
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    /// Remove a key from the map, returning the value if it was previously in the map.
    ///
    /// ```rust
    /// use assoc::AssocListStrictExt;
    ///
    /// let mut map = vec![("a", 1)];
    /// assert_eq!(AssocListStrictExt::remove(&mut map, "a"), Some(1));
    /// assert_eq!(AssocListStrictExt::remove(&mut map, "a"), None);
    /// ```
    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized;
}

impl<K, V> AssocListStrictExt<K, V> for Vec<(K, V)>
where
    K: Eq,
{
    fn entry(&mut self, key: K) -> Entry<K, V> {
        AssocListExt::entry(self, key)
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        AssocListExt::get(self, key)
    }

    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        AssocListExt::get_mut(self, key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        AssocListExt::insert(self, key, value)
    }

    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        AssocListExt::remove(self, key)
    }
}
