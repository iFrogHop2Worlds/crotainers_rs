use std::alloc;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Bound, Index, IndexMut, RangeBounds};
use std::slice;

#[derive(Debug)]
pub struct CroVec<T> {
    pub(crate) data: *mut T,
    pub(crate) size: usize,
    cap: usize,
}

impl<T> Index<usize> for CroVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "Index out of bounds");
        unsafe { &*self.data.add(index) }
    }
}

impl<T> IndexMut<usize> for CroVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size, "Index out of bounds");
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> CroVec<T> {
    pub fn new() -> Self {
        CroVec {
            data: std::ptr::null_mut(),
            size: 0,
            cap: 0,
        }
    }

    pub fn with_cap(cap: usize) -> Self {
        let data = if cap == 0 {
            std::ptr::null_mut()
        } else {
            unsafe {
                std::alloc::alloc(
                    std::alloc::Layout::array::<T>(cap).unwrap()
                ) as *mut T
            }
        };

        CroVec {
            data,
            size: 0,
            cap,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.size {
            unsafe { Some(&*self.data.add(index)) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.size {
            unsafe { Some(&mut *self.data.add(index)) }
        } else {
            None
        }
    }

    pub fn first(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    pub fn last(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            self.get(self.size - 1)
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            None
        } else {
            self.get_mut(self.size - 1)
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.size, "Index out of bounds for insertion");

        if self.size == self.cap {
            self.reserve(1);
        }

        unsafe {
            if index < self.size {
                std::ptr::copy(
                    self.data.add(index),
                    self.data.add(index + 1),
                    self.size - index
                );
            }

            std::ptr::write(self.data.add(index), value);
        }

        self.size += 1;
    }

    pub fn push(&mut self, value: T) {
        if self.size == self.cap {
            self.reserve(1);
        }

        unsafe {
            std::ptr::write(self.data.add(self.size), value);
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            unsafe {
                Some(std::ptr::read(self.data.add(self.size)))
            }
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        if additional == 0 {
            return;
        }

        let required = self.size + additional;
        if required <= self.cap {
            return;
        }

        let mut new_cap = if self.cap == 0 { 1 } else { self.cap };
        while new_cap < required {
            new_cap *= 2;
        }
        self.realloc_to(new_cap);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        let required = self.size + additional;
        if required <= self.cap {
            return;
        }
        self.realloc_to(required);
    }

    pub fn shrink_to_fit(&mut self) {
        self.shrink_to(self.size);
    }

    pub fn shrink_to(&mut self, min_cap: usize) {
        let target = if min_cap < self.size { self.size } else { min_cap };
        if target < self.cap {
            self.realloc_to(target);
        }
    }

    pub fn wipe(&mut self) {
        while self.pop().is_some() {}
    }

    pub fn clear(&mut self) {
        self.wipe();
    }

    pub fn truncate(&mut self, len: usize) {
        if len >= self.size {
            return;
        }

        if self.data.is_null() {
            debug_assert!(self.size == 0);
            return;
        }

        for i in (len..self.size).rev() {
            unsafe {
                std::ptr::drop_in_place(self.data.add(i));
            }
        }

        self.size = len;

        debug_assert!(self.size <= self.cap);
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        assert!(a < self.size && b < self.size, "Index out of bounds");

        if a == b {
            return;
        }

        unsafe {
            let ptr_a = self.data.add(a);
            let ptr_b = self.data.add(b);

            std::ptr::swap(ptr_a, ptr_b);
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.size, "Index out of bounds");

        let old_size = self.size;
        let value = unsafe { std::ptr::read(self.data.add(index)) };

        unsafe {
            if index < self.size - 1 {
                std::ptr::copy(
                    self.data.add(index + 1),
                    self.data.add(index),
                    self.size - index - 1
                );
            }
        }

        self.size -= 1;
        if index < old_size - 1 {
            unsafe {
                std::ptr::drop_in_place(self.data.add(self.size));
            }
        }

        value
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(index < self.size, "Index out of bounds");
        if index == self.size - 1 {
            return self.pop().unwrap();
        }
        let value = unsafe { std::ptr::read(self.data.add(index)) };
        unsafe {
            let last = std::ptr::read(self.data.add(self.size - 1));
            std::ptr::write(self.data.add(index), last);
        }
        self.size -= 1;
        value
    }

    pub fn append(&mut self, other: &mut CroVec<T>) {
        if other.size == 0 {
            return;
        }
        self.reserve(other.size);
        for i in 0..other.size {
            unsafe {
                let value = std::ptr::read(other.data.add(i));
                std::ptr::write(self.data.add(self.size + i), value);
            }
        }
        self.size += other.size;
        other.size = 0;
    }

    pub fn split_off(&mut self, at: usize) -> CroVec<T> {
        assert!(at <= self.size, "Index out of bounds");
        let mut right = CroVec::with_cap(self.size - at);
        for i in at..self.size {
            unsafe {
                let value = std::ptr::read(self.data.add(i));
                right.push(value);
            }
        }
        self.size = at;
        right
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut write = 0;
        for read in 0..self.size {
            unsafe {
                let item = &*self.data.add(read);
                if f(item) {
                    if write != read {
                        let value = std::ptr::read(self.data.add(read));
                        std::ptr::write(self.data.add(write), value);
                    }
                    write += 1;
                } else {
                    std::ptr::drop_in_place(self.data.add(read));
                }
            }
        }
        self.size = write;
    }

    pub fn dedup(&mut self)
    where
        T: PartialEq,
    {
        self.dedup_by(|a, b| a == b);
    }

    pub fn dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        if self.size <= 1 {
            return;
        }
        let mut write = 1;
        for read in 1..self.size {
            unsafe {
                let prev = self.data.add(write - 1);
                let curr = self.data.add(read);
                if same_bucket(&mut *prev, &mut *curr) {
                    std::ptr::drop_in_place(curr);
                } else {
                    if write != read {
                        let value = std::ptr::read(curr);
                        std::ptr::write(self.data.add(write), value);
                    }
                    write += 1;
                }
            }
        }
        self.size = write;
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.size {
            if &self[i] == value {
                return true;
            }
        }
        false
    }

    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        if new_len < self.size {
            self.truncate(new_len);
        } else if new_len > self.size {
            let additional = new_len - self.size;
            self.reserve(additional);
            for _ in 0..additional {
                self.push(value.clone());
            }
        }
    }

    pub fn resize_with<F>(&mut self, new_len: usize, mut f: F)
    where
        F: FnMut() -> T,
    {
        if new_len < self.size {
            self.truncate(new_len);
        } else {
            let additional = new_len - self.size;
            self.reserve(additional);
            for _ in 0..additional {
                self.push(f());
            }
        }
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for value in iter {
            self.push(value);
        }
    }

    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.reserve(other.len());
        for value in other {
            self.push(value.clone());
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data, self.size) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data, self.size) }
    }

    pub fn as_ptr(&self) -> *const T {
        self.data as *const T
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }

    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.as_slice().binary_search(x)
    }

    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        self.as_slice().binary_search_by(f)
    }

    pub fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        self.as_slice().binary_search_by_key(b, f)
    }

    pub fn drain<R>(&mut self, range: R) -> CroVecIntoIter<T>
    where
        R: RangeBounds<usize>,
    {
        let (start, end) = resolve_range(range, self.size);
        assert!(start <= end, "Invalid drain range");
        assert!(end <= self.size, "Drain range out of bounds");

        let old_size = self.size;
        let mut drained = CroVec::with_cap(end - start);
        for i in start..end {
            unsafe {
                let value = std::ptr::read(self.data.add(i));
                drained.push(value);
            }
        }

        if end < self.size {
            unsafe {
                std::ptr::copy(
                    self.data.add(end),
                    self.data.add(start),
                    self.size - end,
                );
            }
        }
        self.size -= end - start;
        for i in self.size..old_size {
            unsafe {
                std::ptr::drop_in_place(self.data.add(i));
            }
        }

        drained.into_iter()
    }

    fn realloc_to(&mut self, new_cap: usize) {
        debug_assert!(new_cap >= self.size);
        if new_cap == self.cap {
            return;
        }
        if new_cap == 0 {
            if !self.data.is_null() {
                unsafe {
                    let old_layout = alloc::Layout::array::<T>(self.cap).unwrap();
                    alloc::dealloc(self.data as *mut u8, old_layout);
                }
            }
            self.data = std::ptr::null_mut();
            self.cap = 0;
            return;
        }

        let new_data = unsafe {
            let layout = alloc::Layout::array::<T>(new_cap).unwrap();
            let ptr = alloc::alloc(layout) as *mut T;
            if !self.data.is_null() {
                std::ptr::copy_nonoverlapping(self.data, ptr, self.size);
                let old_layout = alloc::Layout::array::<T>(self.cap).unwrap();
                alloc::dealloc(self.data as *mut u8, old_layout);
            }
            ptr
        };

        self.data = new_data;
        self.cap = new_cap;
    }
}

impl<T> Drop for CroVec<T> {
    fn drop(&mut self) {
        self.wipe();
        if !self.data.is_null() {
            unsafe {
                std::alloc::dealloc(
                    self.data as *mut u8,
                    std::alloc::Layout::array::<T>(self.cap).unwrap()
                );
            }
        }
    }
}

pub struct CroVecIter<'a, T> {
    vec: &'a CroVec<T>,
    index: usize,
    _phantom: PhantomData<&'a T>,
}


pub struct CroVecIterMut<'a, T> {
    vec: &'a mut CroVec<T>,
    index: usize,
    _phantom: PhantomData<&'a mut T>,
}

pub struct CroVecIntoIter<T> {
    data: *mut T,
    index: usize,
    len: usize,
    cap: usize,
}

impl<T> CroVec<T> {
    pub fn iter(&self) -> CroVecIter<'_, T> {
        CroVecIter {
            vec: self,
            index: 0,
            _phantom: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> CroVecIterMut<'_, T> {
        CroVecIterMut {
            vec: self,
            index: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for CroVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.size {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vec.size - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a, T> Iterator for CroVecIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.size {
            let item = unsafe { &mut *self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vec.size - self.index;
        (remaining, Some(remaining))
    }
}

impl<T> Iterator for CroVecIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { std::ptr::read(self.data.add(self.index)) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T> Drop for CroVecIntoIter<T> {
    fn drop(&mut self) {
        for i in self.index..self.len {
            unsafe {
                std::ptr::drop_in_place(self.data.add(i));
            }
        }
        if !self.data.is_null() && self.cap > 0 {
            unsafe {
                alloc::dealloc(
                    self.data as *mut u8,
                    alloc::Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}

impl<T> Default for CroVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for CroVec<T> {
    fn clone(&self) -> Self {
        let mut out = CroVec::with_cap(self.size);
        for i in 0..self.size {
            out.push(self[i].clone());
        }
        out
    }
}

impl<T: PartialEq> PartialEq for CroVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq> Eq for CroVec<T> {}

impl<T: PartialOrd> PartialOrd for CroVec<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord> Ord for CroVec<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: Hash> Hash for CroVec<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T> Extend<T> for CroVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        CroVec::extend(self, iter);
    }
}

impl<T> FromIterator<T> for CroVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut out = CroVec::new();
        out.extend(iter);
        out
    }
}

impl<'a, T> IntoIterator for &'a CroVec<T> {
    type Item = &'a T;
    type IntoIter = CroVecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut CroVec<T> {
    type Item = &'a mut T;
    type IntoIter = CroVecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> IntoIterator for CroVec<T> {
    type Item = T;
    type IntoIter = CroVecIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = std::mem::ManuallyDrop::new(self);
        let data = vec.data;
        let len = vec.size;
        let cap = vec.cap;
        CroVecIntoIter {
            data,
            index: 0,
            len,
            cap,
        }
    }
}

impl<T: PartialOrd> CroVec<T> {
    pub fn sort(&mut self) {
        if self.size <= 1 {
            return;
        }
        // Allocate temporary buffer for merge sort
        let mut temp = CroVec::with_cap(self.size);
        unsafe {
            // Initialize size to match capacity for proper deallocation
            temp.size = temp.cap;
            self.merge_sort(0, self.size - 1, &mut temp);
            // Reset size to 0 to prevent double-free
            temp.size = 0;
        }
    }

    unsafe fn merge_sort(&mut self, left: usize, right: usize, temp: &mut CroVec<T>) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;

        // Sort left and right halves
        self.merge_sort(left, mid, temp);
        self.merge_sort(mid + 1, right, temp);

        // Merge the sorted halves
        self.merge(left, mid, right, temp);
    }

    unsafe fn merge(&mut self, left: usize, mid: usize, right: usize, temp: &mut CroVec<T>) {
        let mut i = left;
        let mut j = mid + 1;
        let mut k = left;

        // Copy elements to temporary buffer
        while i <= mid && j <= right {
            if (*self.data.add(i)).le(&*self.data.add(j)) {
                std::ptr::copy_nonoverlapping(
                    self.data.add(i),
                    temp.data.add(k),
                    1
                );
                i += 1;
            } else {
                std::ptr::copy_nonoverlapping(
                    self.data.add(j),
                    temp.data.add(k),
                    1
                );
                j += 1;
            }
            k += 1;
        }

        // Copy remaining elements from left half
        while i <= mid {
            std::ptr::copy_nonoverlapping(
                self.data.add(i),
                temp.data.add(k),
                1
            );
            i += 1;
            k += 1;
        }

        // Copy remaining elements from right half
        while j <= right {
            std::ptr::copy_nonoverlapping(
                self.data.add(j),
                temp.data.add(k),
                1
            );
            j += 1;
            k += 1;
        }

        // Copy back from temp to original array
        std::ptr::copy_nonoverlapping(
            temp.data.add(left),
            self.data.add(left),
            right - left + 1
        );
    }

    pub fn sort_unstable(&mut self) {
        self.sort();
    }
}

fn resolve_range<R: RangeBounds<usize>>(range: R, len: usize) -> (usize, usize) {
    let start = match range.start_bound() {
        Bound::Included(&s) => s,
        Bound::Excluded(&s) => s + 1,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(&e) => e + 1,
        Bound::Excluded(&e) => e,
        Bound::Unbounded => len,
    };
    (start, end)
}
