pub trait AssocListExt<K, V> {
    fn entry(&mut self, key: K) -> Entry<K, V>;
}

impl<K, V> AssocListExt<K, V> for Vec<(K, V)>
where
    K: PartialEq,
{
    fn entry(&mut self, key: K) -> Entry<K, V> {
        for (i, (k, _)) in self.iter_mut().enumerate() {
            if k == &key {
                return Entry::Occupied(self, key, i);
            }
        }
        Entry::Vacant(self, key)
    }
}

use std::collections::{LinkedList, VecDeque};
trait Pushable<T> {
    fn push(&mut self, t: T);
    fn last_mut(&mut self) -> Option<&mut T>;
}

impl<T> Pushable<T> for Vec<T> {
    fn push(&mut self, t: T) {
        Vec::push(self, t)
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        <[T]>::last_mut(self)
    }
}

impl<T> Pushable<T> for VecDeque<T> {
    fn push(&mut self, t: T) {
        VecDeque::push_back(self, t)
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        VecDeque::back_mut(self)
    }
}

impl<T> Pushable<T> for LinkedList<T> {
    fn push(&mut self, t: T) {
        LinkedList::push_back(self, t)
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        LinkedList::back_mut(self)
    }
}

pub enum Entry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    Vacant(&'a mut Vec<(K, V)>, K),
    Occupied(&'a mut Vec<(K, V)>, K, usize),
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    /// Ensures a value is in the entry by inserting the default if it is empty, and returns a
    /// mutable reference to the value in the entry.
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut v = vec![("a", 1), ("b", 2)];
    /// v.entry("c").or_insert(3);
    ///
    /// assert_eq!(v, vec![("a", 1), ("b", 2), ("c", 3)]);
    /// ```
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Vacant(list, key) => {
                list.push((key, default));
                let (_, v) = list.last_mut().unwrap();
                v
            }
            Entry::Occupied(list, _, index) => {
                let (_, v) = list.get_mut(index).unwrap();
                v
            }
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut v = vec![("a", 1), ("b", 2)];
    /// v.entry("c").or_insert_with(|| 3);
    ///
    /// assert_eq!(v, vec![("a", 1), ("b", 2), ("c", 3)]);
    /// ```
    pub fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Vacant(_, _) => self.or_insert(default()),
            Entry::Occupied(list, _, index) => {
                let (_, v) = list.get_mut(index).unwrap();
                v
            }
        }
    }

    /// Ensures a value is in the entry by inserting, if empty, the result of the default function.
    /// This method allows for generating key-derived values for insertion by providing the default
    /// function a reference to the key that was moved during the `.entry(key)` method call.
    /// The reference to the moved key is provided so that cloning or copying the key is
    /// unnecessary, unlike with [`Entry::or_insert_with`].
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut v = vec![("a", 1), ("b", 2)];
    /// v.entry("c").or_insert_with_key(|key| key.len());
    ///
    /// assert_eq!(v, vec![("a", 1), ("b", 2), ("c", 1)]);
    /// ```
    pub fn or_insert_with_key<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            Entry::Vacant(list, key) => {
                let v = default(&key);
                list.push((key, v));
                let (_, v) = list.last_mut().unwrap();
                v
            }
            Entry::Occupied(list, _, index) => {
                let (_, v) = list.get_mut(index).unwrap();
                v
            }
        }
    }

    /// Returns a reference to this entry's key.
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut v = vec![("a", 1), ("b", 2)];
    /// assert_eq!(v.entry("a").key(), &"a");
    /// ```
    pub fn key(&self) -> &K {
        match self {
            Entry::Vacant(_, key) => &key,
            Entry::Occupied(_, key, _) => &key,
        }
    }

    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut v = vec![("a", 1), ("b", 2)];
    /// v.entry("c").and_modify(|e| *e += 1).or_insert(3);
    /// assert_eq!(v, vec![("a", 1), ("b", 2), ("c", 3)]);
    ///
    /// v.entry("c").and_modify(|e| *e += 1).or_insert(3);
    /// assert_eq!(v, vec![("a", 1), ("b", 2), ("c", 4)]);
    /// ```
    pub fn and_modify<F>(self, f: F) -> Entry<'a, K, V>
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Vacant(list, key) => Entry::Vacant(list, key),
            Entry::Occupied(list, key, index) => {
                let (_, v) = list.get_mut(index).unwrap();
                f(v);
                Entry::Occupied(list, key, index)
            }
        }
    }
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: 'a,
    V: 'a + Default,
{
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut v = vec![("a", 1), ("b", 2)];
    /// v.entry("c").or_default();
    /// assert_eq!(v, vec![("a", 1), ("b", 2), ("c", 0)]);
    /// ```
    pub fn or_default(self) -> &'a mut V {
        self.or_insert_with(Default::default)
    }
}
