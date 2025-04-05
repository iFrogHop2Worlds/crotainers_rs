use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct CroLList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T> CroLList<T> {
    pub fn new() -> Self {
        CroLList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
        }

        self.size += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }

        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.size -= 1;

            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }

            Rc::try_unwrap(old_head)
                .ok()
                .unwrap()
                .into_inner()
                .data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            self.size -= 1;

            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }

            Rc::try_unwrap(old_tail)
                .ok()
                .unwrap()
                .into_inner()
                .data
        })
    }

    pub fn front(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.head.as_ref().map(Rc::clone)
    }

    pub fn back(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.tail.as_ref().map(Rc::clone)
    }

    pub fn wipe(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Drop for CroLList<T> {
    fn drop(&mut self) {
        self.wipe();
    }
}

pub struct IntoIter<T>(CroLList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for CroLList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}