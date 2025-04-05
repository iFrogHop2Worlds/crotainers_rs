#[derive(Debug)]
pub struct CroVec<T> {
    data: *mut T,
    size: usize,
    cap: usize,
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