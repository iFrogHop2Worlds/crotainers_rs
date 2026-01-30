use std::cmp::Ordering;
use std::iter::FromIterator;
use std::ops::{Bound, RangeBounds};

use crate::maps::CroBTree;
use crate::sequences::CroVec;

#[derive(Debug)]
pub struct CroBTreeSet<K> {
    tree: CroBTree<K, ()>,
}

pub struct CroBTreeSetIter<'a, K> {
    items: CroVec<&'a K>,
    index: usize,
}

pub struct CroBTreeSetIntoIter<K> {
    items: CroVec<K>,
    index: usize,
}

pub type CroBTreeSetRange<'a, K> = CroBTreeSetIter<'a, K>;
pub type CroBTreeSetUnion<'a, K> = CroBTreeSetIter<'a, K>;
pub type CroBTreeSetIntersection<'a, K> = CroBTreeSetIter<'a, K>;
pub type CroBTreeSetDifference<'a, K> = CroBTreeSetIter<'a, K>;
pub type CroBTreeSetSymmetricDifference<'a, K> = CroBTreeSetIter<'a, K>;

impl<'a, K> Iterator for CroBTreeSetIter<'a, K> {
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

impl<K> Iterator for CroBTreeSetIntoIter<K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.size() {
            let item = self.items.remove(self.index);
            Some(item)
        } else {
            None
        }
    }
}

impl<K> CroBTreeSet<K>
where
    K: Ord + Clone,
{
    pub fn new() -> Self {
        Self {
            tree: CroBTree::new(),
        }
    }

    pub fn with_order(order: usize) -> Self {
        Self {
            tree: CroBTree::with_order(order),
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn wipe(&mut self) {
        self.tree.wipe();
    }

    pub fn insert(&mut self, key: K) -> bool {
        self.tree.insert(key, ()).is_none()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.tree.contains_key(key)
    }

    pub fn get(&self, key: &K) -> Option<&K> {
        for item in self.iter() {
            if item == key {
                return Some(item);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> bool {
        self.take(key).is_some()
    }

    pub fn take(&mut self, key: &K) -> Option<K> {
        let mut removed = None;
        let mut next = CroBTreeSet::with_order(self.tree.order());
        for item in self.iter() {
            if item == key {
                removed = Some(item.clone());
            } else {
                next.insert(item.clone());
            }
        }
        *self = next;
        removed
    }

    pub fn replace(&mut self, key: K) -> Option<K> {
        let old = self.take(&key);
        self.insert(key);
        old
    }

    pub fn first(&self) -> Option<&K> {
        self.iter().next()
    }

    pub fn last(&self) -> Option<&K> {
        let mut iter = self.iter();
        iter.next_back()
    }

    pub fn pop_first(&mut self) -> Option<K> {
        let key = self.first()?.clone();
        self.remove(&key);
        Some(key)
    }

    pub fn pop_last(&mut self) -> Option<K> {
        let key = self.last()?.clone();
        self.remove(&key);
        Some(key)
    }

    pub fn iter(&self) -> CroBTreeSetIter<'_, K> {
        let mut items = CroVec::new();
        for (key, _) in self.tree.iter() {
            items.push(key);
        }
        CroBTreeSetIter { items, index: 0 }
    }

    pub fn range<R>(&self, range: R) -> CroBTreeSetRange<'_, K>
    where
        R: RangeBounds<K>,
    {
        let mut items = CroVec::new();
        for key in self.iter() {
            if in_range(&range, key) {
                items.push(key);
            }
        }
        CroBTreeSetIter { items, index: 0 }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K) -> bool,
    {
        let mut next = CroBTreeSet::with_order(self.tree.order());
        for item in self.iter() {
            if f(item) {
                next.insert(item.clone());
            }
        }
        *self = next;
    }

    pub fn append(&mut self, other: &mut CroBTreeSet<K>) {
        for item in other.iter() {
            self.insert(item.clone());
        }
        other.wipe();
    }

    pub fn split_off(&mut self, key: &K) -> CroBTreeSet<K> {
        let mut left = CroBTreeSet::with_order(self.tree.order());
        let mut right = CroBTreeSet::with_order(self.tree.order());
        for item in self.iter() {
            if item < key {
                left.insert(item.clone());
            } else {
                right.insert(item.clone());
            }
        }
        *self = left;
        right
    }

    pub fn union<'a>(&'a self, other: &'a CroBTreeSet<K>) -> CroBTreeSetUnion<'a, K> {
        let left = self.collect_keys();
        let right = other.collect_keys();
        let mut items = CroVec::new();
        let mut i = 0;
        let mut j = 0;
        while i < left.size() && j < right.size() {
            match left[i].cmp(right[j]) {
                Ordering::Less => {
                    items.push(left[i]);
                    i += 1;
                }
                Ordering::Greater => {
                    items.push(right[j]);
                    j += 1;
                }
                Ordering::Equal => {
                    items.push(left[i]);
                    i += 1;
                    j += 1;
                }
            }
        }
        while i < left.size() {
            items.push(left[i]);
            i += 1;
        }
        while j < right.size() {
            items.push(right[j]);
            j += 1;
        }
        CroBTreeSetIter { items, index: 0 }
    }

    pub fn intersection<'a>(
        &'a self,
        other: &'a CroBTreeSet<K>,
    ) -> CroBTreeSetIntersection<'a, K> {
        let left = self.collect_keys();
        let right = other.collect_keys();
        let mut items = CroVec::new();
        let mut i = 0;
        let mut j = 0;
        while i < left.size() && j < right.size() {
            match left[i].cmp(right[j]) {
                Ordering::Less => i += 1,
                Ordering::Greater => j += 1,
                Ordering::Equal => {
                    items.push(left[i]);
                    i += 1;
                    j += 1;
                }
            }
        }
        CroBTreeSetIter { items, index: 0 }
    }

    pub fn difference<'a>(&'a self, other: &'a CroBTreeSet<K>) -> CroBTreeSetDifference<'a, K> {
        let left = self.collect_keys();
        let right = other.collect_keys();
        let mut items = CroVec::new();
        let mut i = 0;
        let mut j = 0;
        while i < left.size() {
            if j >= right.size() {
                items.push(left[i]);
                i += 1;
                continue;
            }
            match left[i].cmp(right[j]) {
                Ordering::Less => {
                    items.push(left[i]);
                    i += 1;
                }
                Ordering::Greater => {
                    j += 1;
                }
                Ordering::Equal => {
                    i += 1;
                    j += 1;
                }
            }
        }
        CroBTreeSetIter { items, index: 0 }
    }

    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a CroBTreeSet<K>,
    ) -> CroBTreeSetSymmetricDifference<'a, K> {
        let left = self.collect_keys();
        let right = other.collect_keys();
        let mut items = CroVec::new();
        let mut i = 0;
        let mut j = 0;
        while i < left.size() || j < right.size() {
            if i >= left.size() {
                items.push(right[j]);
                j += 1;
                continue;
            }
            if j >= right.size() {
                items.push(left[i]);
                i += 1;
                continue;
            }
            match left[i].cmp(right[j]) {
                Ordering::Less => {
                    items.push(left[i]);
                    i += 1;
                }
                Ordering::Greater => {
                    items.push(right[j]);
                    j += 1;
                }
                Ordering::Equal => {
                    i += 1;
                    j += 1;
                }
            }
        }
        CroBTreeSetIter { items, index: 0 }
    }

    pub fn is_subset(&self, other: &CroBTreeSet<K>) -> bool {
        for item in self.iter() {
            if !other.contains(item) {
                return false;
            }
        }
        true
    }

    pub fn is_superset(&self, other: &CroBTreeSet<K>) -> bool {
        other.is_subset(self)
    }

    pub fn is_disjoint(&self, other: &CroBTreeSet<K>) -> bool {
        for item in self.iter() {
            if other.contains(item) {
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

    fn collect_keys(&self) -> CroVec<&K> {
        let mut items = CroVec::new();
        for item in self.iter() {
            items.push(item);
        }
        items
    }
}

impl<K> Default for CroBTreeSet<K>
where
    K: Ord + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K> Clone for CroBTreeSet<K>
where
    K: Ord + Clone,
{
    fn clone(&self) -> Self {
        let mut next = CroBTreeSet::with_order(self.tree.order());
        for key in self.iter() {
            next.insert(key.clone());
        }
        next
    }
}

impl<K> PartialEq for CroBTreeSet<K>
where
    K: Ord + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut left = self.iter();
        let mut right = other.iter();
        loop {
            match (left.next(), right.next()) {
                (None, None) => return true,
                (Some(a), Some(b)) if a == b => continue,
                _ => return false,
            }
        }
    }
}

impl<K> Eq for CroBTreeSet<K> where K: Ord + Clone {}

impl<K> PartialOrd for CroBTreeSet<K>
where
    K: Ord + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K> Ord for CroBTreeSet<K>
where
    K: Ord + Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let mut left = self.iter();
        let mut right = other.iter();
        loop {
            match (left.next(), right.next()) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(a), Some(b)) => match a.cmp(b) {
                    Ordering::Equal => continue,
                    ordering => return ordering,
                },
            }
        }
    }
}

impl<K> FromIterator<K> for CroBTreeSet<K>
where
    K: Ord + Clone,
{
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let mut set = CroBTreeSet::new();
        set.extend(iter);
        set
    }
}

impl<K> Extend<K> for CroBTreeSet<K>
where
    K: Ord + Clone,
{
    fn extend<I: IntoIterator<Item = K>>(&mut self, iter: I) {
        CroBTreeSet::extend(self, iter);
    }
}

impl<'a, K> IntoIterator for &'a CroBTreeSet<K>
where
    K: Ord + Clone,
{
    type Item = &'a K;
    type IntoIter = CroBTreeSetIter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K> IntoIterator for CroBTreeSet<K>
where
    K: Ord + Clone,
{
    type Item = K;
    type IntoIter = CroBTreeSetIntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        let mut items = CroVec::new();
        for key in self.iter() {
            items.push(key.clone());
        }
        CroBTreeSetIntoIter { items, index: 0 }
    }
}

impl<'a, K> CroBTreeSetIter<'a, K> {
    fn next_back(&mut self) -> Option<&'a K> {
        if self.items.size() == 0 || self.index >= self.items.size() {
            return None;
        }
        let last_index = self.items.size() - 1;
        if self.index > last_index {
            return None;
        }
        let item = self.items[last_index];
        self.items.remove(last_index);
        Some(&*item)
    }
}

fn in_range<K, R>(range: &R, key: &K) -> bool
where
    K: Ord,
    R: RangeBounds<K>,
{
    let lower_ok = match range.start_bound() {
        Bound::Included(start) => key >= start,
        Bound::Excluded(start) => key > start,
        Bound::Unbounded => true,
    };
    let upper_ok = match range.end_bound() {
        Bound::Included(end) => key <= end,
        Bound::Excluded(end) => key < end,
        Bound::Unbounded => true,
    };
    lower_ok && upper_ok
}
