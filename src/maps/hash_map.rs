use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::sequences::vec::CroVec;

#[derive(Debug)]
struct Entry<K, V> {
    key: Option<K>,
    value: Option<V>,
    tombstone: bool,
}

#[derive(Debug)]
pub struct CroMap<K, V> {
    entries: CroVec<Entry<K, V>>,
    size: usize,
}

impl<K, V> Entry<K, V> {
    fn new() -> Self {
        Entry {
            key: None,
            value: None,
            tombstone: false,
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.key = Some(key);
        self.value = Some(value);
        self.tombstone = false;
    }
}

impl<K: Hash + Eq + Clone, V: Clone> CroMap<K, V> {
    pub fn new() -> Self {
        let mut map = CroMap {
            entries: CroVec::with_cap(16),
            size: 0,
        };
        for _ in 0..16 {
            map.entries.push(Entry::new());
        }
        map
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn find_slot(&self, key: &K) -> usize {
        let hash = self.hash(key);
        let cap = self.entries.cap();
        let mut index = hash % cap;
        let mut first_tombstone = None;

        loop {
            let entry = unsafe { &*self.entries.data.add(index) };

            if entry.key.is_none() && !entry.tombstone {
                return first_tombstone.unwrap_or(index);
            }

            if entry.tombstone && first_tombstone.is_none() {
                first_tombstone = Some(index);
            }

            if let Some(ref existing_key) = entry.key {
                if !entry.tombstone && existing_key == key {
                    return index;
                }
            }

            index = (index + 1) % cap;
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.size >= self.entries.cap() / 4 {
            self.map_alloc();
        }

        let index = self.find_slot(&key);
        let entry = unsafe { &mut *self.entries.data.add(index) };

        let old_value = if entry.key.is_some() && !entry.tombstone {
            entry.value.take()
        } else {
            self.size += 1;
            None
        };

        entry.insert(key, value);
        old_value
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.find_slot(key);
        let entry = unsafe { &*self.entries.data.add(index) };

        if entry.key.is_some() && !entry.tombstone {
            entry.value.as_ref()
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.find_slot(key);
        let entry = unsafe { &mut *self.entries.data.add(index) };

        if entry.key.is_some() && !entry.tombstone {
            entry.tombstone = true;
            self.size -= 1;
            entry.value.take()
        } else {
            None
        }
    }

    fn map_alloc(&mut self) {
        let old_cap = self.entries.cap();
        let old_entries = std::mem::replace(&mut self.entries, CroVec::with_cap(old_cap * 4));
        self.size = 0;

        for _ in 0..self.entries.cap() {
            self.entries.push(Entry::new());
        }

        for i in 0..old_entries.cap() {
            unsafe {
                let entry = &*old_entries.data.add(i);
                if let (Some(ref key), Some(ref value)) = (&entry.key, &entry.value) {
                    if !entry.tombstone {
                        self.insert(key.clone(), value.clone());
                    }
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<K, V> Drop for CroMap<K, V> {
    fn drop(&mut self) {
        self.entries.wipe();
    }
}