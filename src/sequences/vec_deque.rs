use crate::sequences::vec::CroVec;

#[derive(Debug)]
pub struct CroQue<T> {
    buffer: CroVec<T>,
    head: usize,
}

impl<T: std::fmt::Debug> CroQue<T> {
    pub fn new() -> Self {
        CroQue {
            buffer: CroVec::new(),
            head: 0,
        }
    }

    pub fn with_cap(cap: usize) -> Self {
        CroQue {
            buffer: CroVec::with_cap(cap),
            head: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.buffer.size()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn cap(&self) -> usize {
        self.buffer.cap()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.debug_print("Before pop_front");
            let value = unsafe {
                Some(std::ptr::read(self.buffer.data.add(self.head)))
            };
            self.buffer.size -= 1;

            if self.buffer.size() == 0 {
                self.head = 0;
            } else {
                // Move remaining elements left
                unsafe {
                    std::ptr::copy(
                        self.buffer.data.add(self.head + 1),
                        self.buffer.data.add(self.head),
                        self.buffer.size()
                    );
                }
            }

            self.debug_print("After pop_front");
            value
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.debug_print("Before push_front");
        if self.buffer.cap() == self.buffer.size() {
            self.buffer.reserve(1);
        }

        unsafe {
            std::ptr::copy(
                self.buffer.data,
                self.buffer.data.add(1),
                self.buffer.size()
            );
            std::ptr::write(self.buffer.data, value);
        }
        self.buffer.size += 1;
        self.debug_print("After push_front");
    }

    pub fn push_back(&mut self, value: T) {
        self.debug_print("Before push_back");
        self.buffer.push(value);
        self.debug_print("After push_back");
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.debug_print("Before pop_back");
        let result = self.buffer.pop();
        self.debug_print("After pop_back");
        result
    }

    pub fn wipe(&mut self) {
        if !self.is_empty() {
            unsafe {
                for i in 0..self.buffer.size() {
                    std::ptr::drop_in_place(self.buffer.data.add(i));
                }
            }
        }

        self.buffer.size = 0;
        self.head = 0;
    }

    fn debug_print(&self, operation: &str) {
        unsafe {
            print!("{}: head={}, size={}, elements=[", operation, self.head, self.buffer.size());
            for i in 0..self.buffer.size() {
                print!("{:?}", *self.buffer.data.add(i));
                if i < self.buffer.size() - 1 {
                    print!(", ");
                }
            }
            println!("]");
        }
    }
}

impl<T> Drop for CroQue<T> {
    fn drop(&mut self) {
        self.buffer.wipe();
    }
}
