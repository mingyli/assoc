pub enum Entry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K: 'a, V: 'a> {
    vec: &'a mut Vec<(K, V)>,
    key: K,
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V> {
    pub(crate) fn new(vec: &'a mut Vec<(K, V)>, key: K) -> VacantEntry<'a, K, V> {
        VacantEntry { vec, key }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn insert(self, v: V) -> &'a mut V {
        self.vec.push((self.key, v));
        let (_, v) = self.vec.last_mut().unwrap();
        v
    }
}

pub struct OccupiedEntry<'a, K, V> {
    vec: &'a mut Vec<(K, V)>,
    key: K,
    index: usize,
}

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a, K, V> {
    pub(crate) fn new(vec: &'a mut Vec<(K, V)>, key: K, index: usize) -> OccupiedEntry<'a, K, V> {
        OccupiedEntry { vec, key, index }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn remove_entry(self) -> (K, V) {
        self.vec.swap_remove(self.index)
    }

    pub fn get(&self) -> &V {
        let (_, v) = &self.vec[self.index];
        v
    }

    pub fn get_mut(&mut self) -> &mut V {
        let (_, v) = &mut self.vec[self.index];
        v
    }

    pub fn into_mut(self) -> &'a mut V {
        let (_, v) = &mut self.vec[self.index];
        v
    }

    pub fn insert(&mut self, mut v: V) -> V {
        std::mem::swap(&mut v, &mut self.vec[self.index].1);
        v
    }

    pub fn remove(self) -> V {
        let (_, v) = self.remove_entry();
        v
    }
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
    /// assert_eq!(v, vec![("a", 1), ("b", 2), ("c", 3)]);
    /// assert_eq!(v.entry("c").or_insert(4), &3);
    /// ```
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Vacant(entry) => entry.insert(default),
            Entry::Occupied(entry) => entry.into_mut(),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// ```rust
    /// use assoc::AssocListExt;
    ///
    /// let mut v = Vec::new();
    /// v.entry("c").or_insert_with(|| 3);
    /// assert_eq!(v, [("c", 3)]);
    /// assert_eq!(v.entry("c").or_insert_with(|| 4), &3);
    /// ```
    pub fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Vacant(_) => self.or_insert(default()),
            Entry::Occupied(entry) => entry.into_mut(),
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
            Entry::Vacant(entry) => {
                let v = default(entry.key());
                entry.insert(v)
            }
            Entry::Occupied(entry) => entry.into_mut(),
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
            Entry::Vacant(entry) => entry.key(),
            Entry::Occupied(entry) => entry.key(),
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
            Entry::Vacant(entry) => Entry::Vacant(entry),
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
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
