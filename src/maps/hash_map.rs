use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::mem::ManuallyDrop;

use crate::sequences::vec::CroVec;

#[derive(Debug)]
struct Entry<K, V> {
    key: Option<K>,
    value: Option<V>,
    tombstone: bool,
}

#[derive(Debug)]
pub struct CroMap<K, V, S = RandomState> {
    entries: CroVec<Entry<K, V>>,
    size: usize,
    build_hasher: S,
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

pub struct CroMapIter<'a, K, V, S> {
    map: &'a CroMap<K, V, S>,
    index: usize,
}

pub struct CroMapIntoIter<K, V, S> {
    entries: CroVec<Entry<K, V>>,
    index: usize,
    _hasher: S,
}

pub struct CroMapDrain<K, V> {
    entries: CroVec<Entry<K, V>>,
    index: usize,
}

impl<'a, K, V, S> Iterator for CroMapIter<'a, K, V, S> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.map.entries.size();
        while self.index < len {
            let index = self.index;
            self.index += 1;
            let entry = unsafe { &*self.map.entries.data.add(index) };
            if !entry.tombstone {
                if let (Some(ref key), Some(ref value)) = (&entry.key, &entry.value) {
                    return Some((key, value));
                }
            }
        }
        None
    }
}

impl<K, V, S> Iterator for CroMapIntoIter<K, V, S> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.entries.size();
        while self.index < len {
            let index = self.index;
            self.index += 1;
            let entry = unsafe { &mut *self.entries.data.add(index) };
            if entry.tombstone {
                continue;
            }
            if let (Some(key), Some(value)) = (entry.key.take(), entry.value.take()) {
                return Some((key, value));
            }
        }
        None
    }
}

impl<K, V> Iterator for CroMapDrain<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.entries.size();
        while self.index < len {
            let index = self.index;
            self.index += 1;
            let entry = unsafe { &mut *self.entries.data.add(index) };
            if entry.tombstone {
                continue;
            }
            if let (Some(key), Some(value)) = (entry.key.take(), entry.value.take()) {
                return Some((key, value));
            }
        }
        None
    }
}

const DEFAULT_CAP: usize = 16;

fn init_entries<K, V>(cap: usize) -> CroVec<Entry<K, V>> {
    let mut entries = CroVec::with_cap(cap);
    for _ in 0..cap {
        entries.push(Entry::new());
    }
    entries
}

fn max_size_for_cap(cap: usize) -> usize {
    cap - (cap / 4)
}

impl<K, V> CroMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        CroMap::with_cap_and_hasher(DEFAULT_CAP, RandomState::new())
    }
}

impl<K, V, S> CroMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub fn with_hasher(build_hasher: S) -> Self {
        Self::with_cap_and_hasher(DEFAULT_CAP, build_hasher)
    }

    pub fn with_cap(cap: usize) -> Self
    where
        S: Default,
    {
        Self::with_cap_and_hasher(cap, S::default())
    }

    pub fn with_cap_and_hasher(cap: usize, build_hasher: S) -> Self {
        let cap = if cap == 0 { 1 } else { cap };
        CroMap {
            entries: init_entries(cap),
            size: 0,
            build_hasher,
        }
    }

    pub fn hasher(&self) -> &S {
        &self.build_hasher
    }

    pub fn cap(&self) -> usize {
        self.entries.cap()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.ensure_capacity(1);

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

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = self.hash(key);
        let index = self.find_slot_with_hash::<Q, _>(hash, |existing| existing.borrow() == key);
        let entry = unsafe { &*self.entries.data.add(index) };

        if entry.key.is_some() && !entry.tombstone {
            entry.value.as_ref()
        } else {
            None
        }
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = self.hash(key);
        let index = self.find_slot_with_hash::<Q, _>(hash, |existing| existing.borrow() == key);
        let entry = unsafe { &*self.entries.data.add(index) };

        if entry.key.is_some() && !entry.tombstone {
            match (&entry.key, &entry.value) {
                (Some(k), Some(v)) => Some((k, v)),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = self.hash(key);
        let index = self.find_slot_with_hash::<Q, _>(hash, |existing| existing.borrow() == key);
        let entry = unsafe { &mut *self.entries.data.add(index) };

        if entry.key.is_some() && !entry.tombstone {
            entry.tombstone = true;
            self.size -= 1;
            if let (Some(k), Some(v)) = (entry.key.take(), entry.value.take()) {
                return Some((k, v));
            }
        }
        None
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.remove_entry(key).map(|(_, value)| value)
    }

    pub fn iter(&self) -> CroMapIter<'_, K, V, S> {
        CroMapIter { map: self, index: 0 }
    }

    pub fn drain(&mut self) -> CroMapDrain<K, V> {
        let cap = self.entries.cap();
        let entries = std::mem::replace(&mut self.entries, init_entries(cap));
        self.size = 0;
        CroMapDrain { entries, index: 0 }
    }

    pub fn wipe(&mut self) {
        for i in 0..self.entries.size() {
            unsafe {
                let entry = &mut *self.entries.data.add(i);
                entry.key.take();
                entry.value.take();
                entry.tombstone = false;
            }
        }
        self.size = 0;
    }

    pub fn reserve(&mut self, additional: usize) {
        self.ensure_capacity(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.shrink_to(1);
    }

    pub fn shrink_to(&mut self, min_cap: usize) {
        let mut cap = if min_cap == 0 { 1 } else { min_cap };
        while self.size > max_size_for_cap(cap) {
            cap *= 2;
        }
        if cap < self.entries.cap() {
            self.rehash(cap);
        }
    }

    fn hash<Q: Hash + ?Sized>(&self, key: &Q) -> usize {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn find_slot(&self, key: &K) -> usize {
        let hash = self.hash(key);
        self.find_slot_with_hash::<K, _>(hash, |existing| existing == key)
    }

    fn find_slot_with_hash<Q: ?Sized, F>(&self, hash: usize, mut matches: F) -> usize
    where
        F: FnMut(&K) -> bool,
    {
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
                if !entry.tombstone && matches(existing_key) {
                    return index;
                }
            }

            index = (index + 1) % cap;
        }
    }

    fn ensure_capacity(&mut self, additional: usize) {
        let required = self.size + additional;
        let mut cap = self.entries.cap();
        if required <= max_size_for_cap(cap) {
            return;
        }
        while required > max_size_for_cap(cap) {
            cap *= 2;
        }
        self.rehash(cap);
    }

    fn rehash(&mut self, new_cap: usize) {
        let old_entries = std::mem::replace(&mut self.entries, init_entries(new_cap));
        self.size = 0;

        for i in 0..old_entries.size() {
            unsafe {
                let entry = &mut *old_entries.data.add(i);
                if entry.tombstone {
                    continue;
                }
                if let (Some(key), Some(value)) = (entry.key.take(), entry.value.take()) {
                    self.insert(key, value);
                }
            }
        }
    }
}

impl<K, V, S> IntoIterator for CroMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    type Item = (K, V);
    type IntoIter = CroMapIntoIter<K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        let mut map = ManuallyDrop::new(self);
        let entries = unsafe { std::ptr::read(&map.entries) };
        let build_hasher = unsafe { std::ptr::read(&map.build_hasher) };
        CroMapIntoIter {
            entries,
            index: 0,
            _hasher: build_hasher,
        }
    }
}

impl<K, V, S> Drop for CroMap<K, V, S> {
    fn drop(&mut self) {
        self.entries.wipe();
    }
}
