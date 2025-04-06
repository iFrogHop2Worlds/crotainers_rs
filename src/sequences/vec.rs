use std::ops::{Index, IndexMut};
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
        // Ensure index is valid
        assert!(index <= self.size, "Index out of bounds for insertion");

        // Ensure we have enough capacity
        if self.size == self.cap {
            let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
            self.reserve(new_cap);
        }

        unsafe {
            // Shift elements to the right to make space for the new element
            if index < self.size {
                std::ptr::copy(
                    self.data.add(index),        // source
                    self.data.add(index + 1),    // destination
                    self.size - index            // number of elements to move
                );
            }

            // Write the new element
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