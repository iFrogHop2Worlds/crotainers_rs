mod etc;
mod maps;
mod sequences;
mod sets;


#[cfg(test)]
mod tests {

    use crate::sequences::{
        CroVec,
        CroQue,
        CroLList
    };

    use crate::maps::{CroBTree, CroMap};
    #[test]
    fn test_new_crovec() {
        let vec: CroVec<i32> = CroVec::new();
        assert_eq!(vec.size(), 0);
        assert_eq!(vec.cap(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_with_cap_vec() {
        let vec: CroVec<i32> = CroVec::with_cap(5);
        assert_eq!(vec.size(), 0);
        assert_eq!(vec.cap(), 5);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_push_pop_vec() {
        let mut vec: CroVec<i32> = CroVec::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.size(), 3);
        assert!(!vec.is_empty());

        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);

        assert!(vec.is_empty());
    }

    #[test]
    fn test_cap_growth_vec() {
        let mut vec: CroVec<i32> = CroVec::with_cap(2);

        vec.push(1);
        vec.push(2);
        assert_eq!(vec.cap(), 2);

        vec.push(3);
        assert_eq!(vec.cap(), 4);
    }

    #[test]
    fn test_reserve_crovec() {
        let mut vec: CroVec<i32> = CroVec::new();

        vec.reserve(5);
        assert_eq!(vec.cap(), 5);

        vec.reserve(3);
        assert_eq!(vec.cap(), 5);

        vec.reserve(10);
        assert_eq!(vec.cap(), 10);
    }

    #[test]
    fn test_wipe_crovec() {
        let mut vec: CroVec<i32> = CroVec::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        vec.wipe();
        assert_eq!(vec.size(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_drop_cleanup_vec() {
        let mut vec: CroVec<String> = CroVec::new();

        vec.push(String::from("test1"));
        vec.push(String::from("test2"));

        drop(vec);
    }

    // double ended que
    #[test]
    fn test_croque_basic() {
        let mut deque = CroQue::new();
        assert!(deque.is_empty());

        deque.push_back(1);
        deque.push_back(2);
        deque.push_front(0);

        assert_eq!(deque.size(), 3);
        assert_eq!(deque.pop_front(), Some(0));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert!(deque.is_empty());
    }

    #[test]
    fn test_croque_cap() {
        let mut deque = CroQue::with_cap(2);
        assert_eq!(deque.cap(), 2);

        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        assert!(deque.cap() > 2);
        assert_eq!(deque.size(), 3);
    }

    #[test]
    fn test_croque_wipe() {
        let mut deque = CroQue::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.wipe();

        assert!(deque.is_empty());
        assert_eq!(deque.pop_front(), None);
        assert_eq!(deque.pop_back(), None);
    }

    // linked list

    #[test]
    fn test_new_crollist() {
        let list: CroLList<i32> = CroLList::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_push_pop_front_ll() {
        let mut list = CroLList::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.size(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_pop_back_ll() {
        let mut list = CroLList::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.size(), 3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_mixed_operations_ll() {
        let mut list = CroLList::new();

        list.push_back(1);
        list.push_front(2);
        list.push_back(3);
        list.push_front(4);

        assert_eq!(list.size(), 4);
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
    }

    #[test]
    fn test_wipe_crollist() {
        let mut list = CroLList::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.wipe();
        assert!(list.is_empty());
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_iterator_ll() {
        let mut list = CroLList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter_values = Vec::new();
        for value in list {
            iter_values.push(value);
        }

        assert_eq!(iter_values, vec![1, 2, 3]);
    }

    // hashmap
    #[test]
    fn test_new_map_is_empty() {
        let map: CroMap<i32, String> = CroMap::new();
        assert!(map.is_empty());
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_basic_insert_and_get() {
        let mut map = CroMap::new();
        map.insert("test", 42);
        assert_eq!(map.get(&"test"), Some(&42));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_update_existing_key() {
        let mut map = CroMap::new();
        map.insert("key", 100);
        assert_eq!(map.insert("key", 200), Some(100));
        assert_eq!(map.get(&"key"), Some(&200));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_remove() {
        let mut map = CroMap::new();
        map.insert("key", 42);
        assert_eq!(map.remove(&"key"), Some(42));
        assert!(map.is_empty());
        assert_eq!(map.get(&"key"), None);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut map: CroMap<&str, i32> = CroMap::new();
        assert_eq!(map.remove(&"nonexistent"), None);
    }

    #[test]
    fn test_multiple_operations() {
        let mut map = CroMap::new();

        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"c"), Some(&3));
        assert_eq!(map.size(), 3);

        assert_eq!(map.remove(&"b"), Some(2));
        assert_eq!(map.size(), 2);

        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), None);
        assert_eq!(map.get(&"c"), Some(&3));

        map.insert("b", 4);
        assert_eq!(map.get(&"b"), Some(&4));
    }

    #[test]
    fn test_resize_behavior() {
        let mut map = CroMap::new();

        for i in 0..10 {
            map.insert(i, i * 10);
        }

        for i in 0..10 {
            assert_eq!(map.get(&i), Some(&(i * 10)));
        }
        assert_eq!(map.size(), 10);
    }

    #[test]
    fn test_insert_after_remove() {
        let mut map = CroMap::new();

        map.insert("key", 1);
        map.remove(&"key");

        map.insert("key", 2);
        assert_eq!(map.get(&"key"), Some(&2));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_complex_collision_scenario() {
        let mut map = CroMap::new();

        let items = vec![
            ("abc", 1),
            ("bac", 2),
            ("cba", 3),
            ("acb", 4),
        ];

        for (k, v) in items.iter() {
            map.insert(*k, *v);
        }

        for (k, v) in items.iter() {
            assert_eq!(map.get(k), Some(v));
        }

        map.remove(&"bac");
        map.remove(&"acb");

        assert_eq!(map.get(&"abc"), Some(&1));
        assert_eq!(map.get(&"cba"), Some(&3));
        assert_eq!(map.get(&"bac"), None);
        assert_eq!(map.get(&"acb"), None);

        map.insert("bac", 5);
        assert_eq!(map.get(&"bac"), Some(&5));
    }

    #[test]
    fn test_large_number_of_operations() {
        let mut map = CroMap::new();
        let num_operations = 1000;

        for i in 0..num_operations {
            map.insert(i, i * 2);
        }

        assert_eq!(map.size(), num_operations);

        for i in 0..num_operations {
            assert_eq!(map.get(&i), Some(&(i * 2)));
        }

        for i in 0..num_operations/2 {
            assert_eq!(map.remove(&i), Some(i * 2));
        }

        assert_eq!(map.size(), num_operations/2);
        for i in num_operations/2..num_operations {
            assert_eq!(map.get(&i), Some(&(i * 2)));
        }
    }

    #[test]
    fn test_edge_cases() {
        let mut map: CroMap<String, i32> = CroMap::new();

        map.insert(String::new(), 1);
        assert_eq!(map.get(&String::new()), Some(&1));

        let long_key = "a".repeat(1000);
        map.insert(long_key.clone(), 2);
        assert_eq!(map.get(&long_key), Some(&2));
    }

    //btreemap
    fn create_test_tree() -> CroBTree<i32, &'static str> {
        let mut tree = CroBTree::new();
        tree.insert(10, "ten");
        tree.insert(20, "twenty");
        tree.insert(5, "five");
        tree
    }

    #[test]
    fn test_new_and_empty() {
        let tree: CroBTree<i32, &str> = CroBTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_custom_order() {
        let tree = CroBTree::<i32, &str>::with_order(4);
        assert_eq!(tree.len(), 0);
    }

    #[test]
    #[should_panic(expected = "B-tree order must be at least 3")]
    fn test_invalid_order() {
        CroBTree::<i32, &str>::with_order(2);
    }

    #[test]
    fn test_insert_and_get() {
        let mut tree = CroBTree::new();

        assert_eq!(tree.insert(1, "one"), None);
        assert_eq!(tree.insert(2, "two"), None);
        assert_eq!(tree.len(), 2);

        assert_eq!(tree.insert(1, "new one"), Some("one"));
        assert_eq!(tree.len(), 2);

        assert_eq!(tree.get(&1), Some(&"new one"));
        assert_eq!(tree.get(&2), Some(&"two"));
        assert_eq!(tree.get(&3), None);
    }

    #[test]
    fn test_get_mut() {
        let mut tree = create_test_tree();

        if let Some(value) = tree.get_mut(&10) {
            *value = "TEN";
        }

        assert_eq!(tree.get(&10), Some(&"TEN"));
    }

    #[test]
    fn test_contains_key() {
        let tree = create_test_tree();

        assert!(tree.contains_key(&5));
        assert!(tree.contains_key(&10));
        assert!(tree.contains_key(&20));
        assert!(!tree.contains_key(&15));
        assert!(!tree.contains_key(&0));
    }

    #[test]
    fn test_clear() {
        let mut tree = create_test_tree();
        assert!(!tree.is_empty());

        tree.wipe();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.get(&10), None);
    }

    #[test]
    fn test_ordering() {
        let mut tree = CroBTree::new();

        tree.insert(30, "thirty");
        tree.insert(10, "ten");
        tree.insert(20, "twenty");
        tree.insert(5, "five");
        tree.insert(15, "fifteen");

        assert_eq!(tree.get(&5), Some(&"five"));
        assert_eq!(tree.get(&10), Some(&"ten"));
        assert_eq!(tree.get(&15), Some(&"fifteen"));
        assert_eq!(tree.get(&20), Some(&"twenty"));
        assert_eq!(tree.get(&30), Some(&"thirty"));
    }

    #[test]
    fn test_large_insertion() {
        let mut tree = CroBTree::new();
        let count = 1000;

        for i in 0..count {
            tree.insert(i, i.to_string());
        }

        assert_eq!(tree.len(), count);

        for i in 0..count {
            assert_eq!(tree.get(&i), Some(&i.to_string()));
        }
    }

    #[test]
    fn test_different_types() {
        let mut string_tree: CroBTree<String, i32> = CroBTree::new();
        string_tree.insert("hello".to_string(), 1);
        string_tree.insert("world".to_string(), 2);

        assert_eq!(string_tree.get(&"hello".to_string()), Some(&1));
        assert_eq!(string_tree.get(&"world".to_string()), Some(&2));

        let mut char_tree: CroBTree<char, bool> = CroBTree::new();
        char_tree.insert('a', true);
        char_tree.insert('b', false);

        assert_eq!(char_tree.get(&'a'), Some(&true));
        assert_eq!(char_tree.get(&'b'), Some(&false));
    }

    #[test]
    fn test_edge_cases_btree() {
        let mut tree = CroBTree::new();

        tree.insert(i32::MIN, "min");
        tree.insert(i32::MAX, "max");
        tree.insert(0, "zero");

        assert_eq!(tree.get(&i32::MIN), Some(&"min"));
        assert_eq!(tree.get(&i32::MAX), Some(&"max"));
        assert_eq!(tree.get(&0), Some(&"zero"));
    }
}
