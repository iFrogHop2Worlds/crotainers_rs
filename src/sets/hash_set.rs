use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;

use crate::maps::{CroMap, CroMapDrain, CroMapIntoIter};
use crate::sequences::CroVec;

#[derive(Debug)]
pub struct CroHashSet<K, S = RandomState> {
    map: CroMap<K, (), S>,
}

pub struct CroHashSetIter<'a, K> {
    items: CroVec<&'a K>,
    index: usize,
}

pub struct CroHashSetIntoIter<K, S> {
    iter: CroMapIntoIter<K, (), S>,
}

pub struct CroHashSetDrain<K> {
    iter: CroMapDrain<K, ()>,
}

pub type CroHashSetUnion<'a, K> = CroHashSetIter<'a, K>;
pub type CroHashSetIntersection<'a, K> = CroHashSetIter<'a, K>;
pub type CroHashSetDifference<'a, K> = CroHashSetIter<'a, K>;
pub type CroHashSetSymmetricDifference<'a, K> = CroHashSetIter<'a, K>;

impl<'a, K> Iterator for CroHashSetIter<'a, K> {
    type Item = &'a K;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.size() {
            let item = self.items[self.index];
            self.index += 1;
            Some(&*item)
        } else {
            None
        }
    }
}

impl<K, S> Iterator for CroHashSetIntoIter<K, S> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, _)| key)
    }
}

impl<K> Iterator for CroHashSetDrain<K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, _)| key)
    }
}

impl<K> CroHashSet<K>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self { map: CroMap::new() }
    }
}

impl<K, S> CroHashSet<K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn with_hasher(build_hasher: S) -> Self {
        Self {
            map: CroMap::with_hasher(build_hasher),
        }
    }

    pub fn with_cap(cap: usize) -> Self
    where
        S: Default,
    {
        Self {
            map: CroMap::with_cap(cap),
        }
    }

    pub fn with_cap_and_hasher(cap: usize, build_hasher: S) -> Self {
        Self {
            map: CroMap::with_cap_and_hasher(cap, build_hasher),
        }
    }

    pub fn hasher(&self) -> &S {
        self.map.hasher()
    }

    pub fn len(&self) -> usize {
        self.map.size()
    }

    pub fn cap(&self) -> usize {
        self.map.cap()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn wipe(&mut self) {
        self.map.wipe();
    }

    pub fn insert(&mut self, key: K) -> bool {
        self.map.insert(key, ()).is_none()
    }

    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get(key).is_some()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get_key_value(key).map(|(k, _)| k)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.remove_entry(key).is_some()
    }

    pub fn take<Q>(&mut self, key: &Q) -> Option<K>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.remove_entry(key).map(|(k, _)| k)
    }

    pub fn replace(&mut self, key: K) -> Option<K> {
        let old = self.take(&key);
        self.insert(key);
        old
    }

    pub fn iter(&self) -> CroHashSetIter<'_, K> {
        let mut items = CroVec::new();
        for (key, _) in self.map.iter() {
            items.push(key);
        }
        CroHashSetIter { items, index: 0 }
    }

    pub fn drain(&mut self) -> CroHashSetDrain<K> {
        CroHashSetDrain {
            iter: self.map.drain(),
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K) -> bool,
    {
        for (key, _) in self.map.drain() {
            if f(&key) {
                self.map.insert(key, ());
            }
        }
    }

    pub fn append(&mut self, other: &mut CroHashSet<K, S>) {
        for (key, _) in other.map.drain() {
            self.insert(key);
        }
    }

    pub fn union<'a>(&'a self, other: &'a CroHashSet<K, S>) -> CroHashSetUnion<'a, K> {
        let mut items = CroVec::new();
        for key in self.iter() {
            items.push(key);
        }
        for key in other.iter() {
            if !self.contains(key) {
                items.push(key);
            }
        }
        CroHashSetIter { items, index: 0 }
    }

    pub fn intersection<'a>(
        &'a self,
        other: &'a CroHashSet<K, S>,
    ) -> CroHashSetIntersection<'a, K> {
        let mut items = CroVec::new();
        for key in self.iter() {
            if other.contains(key) {
                items.push(key);
            }
        }
        CroHashSetIter { items, index: 0 }
    }

    pub fn difference<'a>(
        &'a self,
        other: &'a CroHashSet<K, S>,
    ) -> CroHashSetDifference<'a, K> {
        let mut items = CroVec::new();
        for key in self.iter() {
            if !other.contains(key) {
                items.push(key);
            }
        }
        CroHashSetIter { items, index: 0 }
    }

    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a CroHashSet<K, S>,
    ) -> CroHashSetSymmetricDifference<'a, K> {
        let mut items = CroVec::new();
        for key in self.iter() {
            if !other.contains(key) {
                items.push(key);
            }
        }
        for key in other.iter() {
            if !self.contains(key) {
                items.push(key);
            }
        }
        CroHashSetIter { items, index: 0 }
    }

    pub fn is_subset(&self, other: &CroHashSet<K, S>) -> bool {
        for key in self.iter() {
            if !other.contains(key) {
                return false;
            }
        }
        true
    }

    pub fn is_superset(&self, other: &CroHashSet<K, S>) -> bool {
        other.is_subset(self)
    }

    pub fn is_disjoint(&self, other: &CroHashSet<K, S>) -> bool {
        for key in self.iter() {
            if other.contains(key) {
                return false;
            }
        }
        true
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = K>,
    {
        for key in iter {
            self.insert(key);
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.map.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.map.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_cap: usize) {
        self.map.shrink_to(min_cap);
    }

}

impl<K, S> Default for CroHashSet<K, S>
where
    K: Eq + Hash,
    S: Default + BuildHasher,
{
    fn default() -> Self {
        Self::with_hasher(S::default())
    }
}

impl<K, S> Clone for CroHashSet<K, S>
where
    K: Eq + Hash + Clone,
    S: BuildHasher + Clone,
{
    fn clone(&self) -> Self {
        let mut next = CroHashSet::with_hasher(self.map.hasher().clone());
        for key in self.iter() {
            next.insert(key.clone());
        }
        next
    }
}

impl<K, S> PartialEq for CroHashSet<K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.is_subset(other)
    }
}

impl<K, S> Eq for CroHashSet<K, S> where K: Eq + Hash, S: BuildHasher {}

impl<K, S> FromIterator<K> for CroHashSet<K, S>
where
    K: Eq + Hash,
    S: Default + BuildHasher,
{
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let mut set = CroHashSet::with_hasher(S::default());
        set.extend(iter);
        set
    }
}

impl<K, S> Extend<K> for CroHashSet<K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn extend<I: IntoIterator<Item = K>>(&mut self, iter: I) {
        CroHashSet::extend(self, iter);
    }
}

impl<'a, K, S> IntoIterator for &'a CroHashSet<K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = &'a K;
    type IntoIter = CroHashSetIter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, S> IntoIterator for CroHashSet<K, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = K;
    type IntoIter = CroHashSetIntoIter<K, S>;

    fn into_iter(self) -> Self::IntoIter {
        CroHashSetIntoIter { iter: self.map.into_iter() }
    }
}
