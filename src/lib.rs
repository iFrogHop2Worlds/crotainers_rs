mod etc;
pub mod maps;
mod sequences;
mod sets;


#[cfg(test)]
mod tests {

    use crate::sequences::{vec::CroVec, vec_deque::CroQue, linked_list::CroLList};
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
}
