use crate::vec::{Entry, OccupiedEntry, VacantEntry};

pub trait AssocListExt<K, V> {
    fn entry(&mut self, key: K) -> Entry<K, V>;
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
}
