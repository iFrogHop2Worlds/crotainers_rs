use std::mem;
use crate::sequences::CroVec;

#[derive(Debug, Clone)]
struct Node<K, V> {
    keys: CroVec<K>,
    values: CroVec<V>,
    children: CroVec<Node<K, V>>,
    is_leaf: bool,
}

#[derive(Debug)]
pub struct CroBTree<K, V> {
    root: Option<Node<K, V>>,
    order: usize,
    length: usize,
}

impl<K, V> Node<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn new(is_leaf: bool, order: usize) -> Self {
        let mut keys = CroVec::new();
        let mut values = CroVec::new();
        let mut children = CroVec::new();

        keys.reserve(order - 1);
        values.reserve(order - 1);
        if !is_leaf {
            children.reserve(order);
        }

        Node {
            keys,
            values,
            children,
            is_leaf,
        }
    }

    fn is_full(&self, order: usize) -> bool {
        debug_assert!(self.keys.size() <= order - 1);
        self.keys.size() == order - 1
    }
}

impl<K, V> CroBTree<K, V>
where
    K: Ord + Clone, V: Clone,
    V: Clone
{
    pub fn new() -> Self {
        Self::with_order(6)
    }

    pub fn with_order(order: usize) -> Self {
        assert!(order >= 3, "B-tree order must be at least 3");
        CroBTree {
            root: None,
            order,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn wipe(&mut self) {
        self.root = None;
        self.length = 0;
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.root.is_none() {
            self.root = Some(Node::new(true, self.order));
        }

        let is_root_full = self.root.as_ref().unwrap().is_full(self.order);

        if is_root_full {
            let mut new_root = Node::new(false, self.order);
            let old_root = self.root.take().unwrap();
            new_root.children.push(old_root);
            
            self.root = Some(new_root);

            if let Some(root) = self.root.as_mut() {
                let order = self.order;
                let child = &mut root.children[0];
                let mid = (order - 1) / 2;

                let mut new_node = Node::new(child.is_leaf, order);
                
                for _ in 0..(order - 1 - mid) {
                    new_node.keys.push(child.keys.pop().unwrap());
                    new_node.values.push(child.values.pop().unwrap());
                }
                
                if !child.is_leaf {
                    for _ in 0..order - mid {
                        new_node.children.push(child.children.pop().unwrap());
                    }
                }
                
                root.keys.insert(0, child.keys.pop().unwrap());
                root.values.insert(0, child.values.pop().unwrap());
                root.children.insert(1, new_node);
            }
        }

        let mut root = self.root.take().unwrap();
        let order = self.order;
        let result = self.insert_non_full(&mut root, key, value, order);
        self.root = Some(root);

        if result.is_none() {
            self.length += 1;
        }
        result
    }

    fn insert_non_full(&mut self, node: &mut Node<K, V>, key: K, value: V, order: usize) -> Option<V> {
        let mut i = node.keys.size();

        if node.is_leaf {
            
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }

            
            if i > 0 && node.keys[i - 1] == key {
                return Some(mem::replace(&mut node.values[i - 1], value));
            }

           
            node.keys.insert(i, key);
            node.values.insert(i, value);
            None
        } else {
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }

            if i > 0 && node.keys[i - 1] == key {
                return Some(mem::replace(&mut node.values[i - 1], value));
            }

            let child = &mut node.children[i];
            
            if child.is_full(order) {
                self.split_child(node, i);
                if key > node.keys[i] {
                    i += 1;
                }
            }
            self.insert_non_full(&mut node.children[i], key, value, order)
        }

    }

    fn split_child(&mut self, parent: &mut Node<K, V>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let mid = (self.order - 1) / 2;

        let mut new_node = Node::new(child.is_leaf, self.order);
        
        for i in (mid + 1)..child.keys.size() {
            new_node.keys.push(child.keys[i].clone());
            new_node.values.push(child.values[i].clone());
        }
        
        while child.keys.size() > mid + 1 {
            child.keys.pop();
            child.values.pop();
        }
        
        if !child.is_leaf {
            for i in (mid + 1)..child.children.size() {
                new_node.children.push(child.children[i].clone());
            }
            while child.children.size() > mid + 1 {
                child.children.pop();
            }
        }
        
        let mid_key = child.keys.pop().unwrap();
        let mid_value = child.values.pop().unwrap();

        parent.keys.insert(child_index, mid_key);
        parent.values.insert(child_index, mid_value);
        parent.children.insert(child_index + 1, new_node);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match &self.root {
            None => None,
            Some(node) => self.get_in_node(node, key),
        }
    }

    fn get_in_node<'a>(&self, node: &'a Node<K, V>, key: &K) -> Option<&'a V> {
        let mut i = 0;
        while i < node.keys.size() && key > &node.keys[i] {
            i += 1;
        }

        if i < node.keys.size() && key == &node.keys[i] {
            Some(&node.values[i])
        } else if node.is_leaf {
            None
        } else {
            self.get_in_node(&node.children[i], key)
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match &mut self.root {
            None => None,
            Some(node) => Self::get_mut_in_node(node, key),
        }
    }

    fn get_mut_in_node<'a>(node: &'a mut Node<K, V>, key: &K) -> Option<&'a mut V> {
        let mut i = 0;
        while i < node.keys.size() && key > &node.keys[i] {
            i += 1;
        }

        if i < node.keys.size() && key == &node.keys[i] {
            Some(&mut node.values[i])
        } else if node.is_leaf {
            None
        } else {
            Self::get_mut_in_node(&mut node.children[i], key)
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

impl<K, V> Drop for Node<K, V> {
    fn drop(&mut self) {
        self.keys.wipe();
        self.values.wipe();
        self.children.wipe();
    }
}

impl<K, V> Drop for CroBTree<K, V> {
    fn drop(&mut self) {
        self.root.take();
        self.length = 0;
    }
}