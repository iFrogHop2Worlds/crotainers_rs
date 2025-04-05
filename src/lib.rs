mod etc;
pub mod maps;
mod sequences;
mod sets;


#[cfg(test)]
mod tests {
    use crate::sequences::vec::CroVec;
    #[test]
    fn test_new() {
        let vec: CroVec<i32> = CroVec::new();
        assert_eq!(vec.size(), 0);
        assert_eq!(vec.cap(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let vec: CroVec<i32> = CroVec::with_cap(5);
        assert_eq!(vec.size(), 0);
        assert_eq!(vec.cap(), 5);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_push_pop() {
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
    fn test_capacity_growth() {
        let mut vec: CroVec<i32> = CroVec::with_cap(2);

        vec.push(1);
        vec.push(2);
        assert_eq!(vec.cap(), 2);

        vec.push(3);
        assert_eq!(vec.cap(), 4);
    }

    #[test]
    fn test_reserve() {
        let mut vec: CroVec<i32> = CroVec::new();

        vec.reserve(5);
        assert_eq!(vec.cap(), 5);

        vec.reserve(3);
        assert_eq!(vec.cap(), 5);

        vec.reserve(10);
        assert_eq!(vec.cap(), 10);
    }

    #[test]
    fn test_wipe() {
        let mut vec: CroVec<i32> = CroVec::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        vec.wipe();
        assert_eq!(vec.size(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_drop_cleanup() {
        let mut vec: CroVec<String> = CroVec::new();

        vec.push(String::from("test1"));
        vec.push(String::from("test2"));

        drop(vec);
    }
}
