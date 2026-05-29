use crate::sequences::CroVec;

#[derive(Debug, Clone, Default)]
pub struct BinCroHeap<T> {
    data: CroVec<T>,
}

impl<T: Ord> BinCroHeap<T> {
    pub fn new() -> Self {
        Self { data: CroVec::new() }
    }

    pub fn with_cap(cap: usize) -> Self {
        Self {
            data: CroVec::with_cap(cap),
        }
    }

    pub fn from_crovec(mut vec: CroVec<T>) -> Self {
        if vec.len() > 1 {
            for i in (0..(vec.len() / 2)).rev() {
                Self::sift_down_from(&mut vec, i);
            }
        }
        Self { data: vec }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn size(&self) -> usize {
        self.len()
    }

    pub fn cap(&self) -> usize {
        self.data.cap()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.first_mut()
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        let last = self.data.len() - 1;
        self.sift_up(last);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        Some(self.data.swap_remove(0)).inspect(|_| {
            if !self.data.is_empty() {
                self.sift_down(0);
            }
        })
    }

    pub fn clear(&mut self) {
        self.data.wipe();
    }

    pub fn wipe(&mut self) {
        self.clear();
    }

    pub fn into_crovec(self) -> CroVec<T> {
        self.data
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[parent] >= self.data[idx] {
                break;
            }
            self.data.swap(parent, idx);
            idx = parent;
        }
    }

    fn sift_down(&mut self, idx: usize) {
        Self::sift_down_from(&mut self.data, idx);
    }

    fn sift_down_from(data: &mut CroVec<T>, mut idx: usize) {
        let len = data.len();
        loop {
            let left = idx * 2 + 1;
            if left >= len {
                break;
            }

            let right = left + 1;
            let mut largest = left;
            if right < len && data[right] > data[left] {
                largest = right;
            }

            if data[idx] >= data[largest] {
                break;
            }

            data.swap(idx, largest);
            idx = largest;
        }
    }
}
