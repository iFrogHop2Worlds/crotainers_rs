use std::ops::{Index, IndexMut};
use std::marker::PhantomData;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.size, "Index out of bounds for insertion");

        if self.size == self.cap {
            let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
            self.reserve(new_cap);
        }

        unsafe {
            if index < self.size {
                std::ptr::copy(
                    self.data.add(index),
                    self.data.add(index + 1),
                );
            }

            std::ptr::write(self.data.add(index), value);
        }

        self.size += 1;
    }

    pub fn push(&mut self, value: T) {
        if self.size == self.cap {
            let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
            self.reserve(new_cap);
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

    pub fn reserve(&mut self, new_cap: usize) {
        if new_cap <= self.cap {
            return;
        }

        let new_data = unsafe {
            let layout = std::alloc::Layout::array::<T>(new_cap).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut T;

            if !self.data.is_null() {
                std::ptr::copy_nonoverlapping(self.data, ptr, self.size);
                let old_layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
                std::alloc::dealloc(self.data as *mut u8, old_layout);
            }

            ptr
        };

        self.data = new_data;
        self.cap = new_cap;
    }

    pub fn wipe(&mut self) {
        while self.pop().is_some() {}
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

        value
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
            Some(unsafe { &mut *(item as *mut T) })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vec.size - self.index;
        (remaining, Some(remaining))
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
}