mod tests_i32 {
    use rust_group_project::{dll_trait::DoubleLinkedList, rc_refcell::Dll};

    /**
     * dll = [0, 1, 2, 3, 4]
     */
    fn generate_dll() -> Box<dyn DoubleLinkedList<i32>> {
        let mut dll = Dll::default();
        dll.push_back(0);
        dll.push_back(1);
        dll.push_back(2);
        dll.push_back(3);
        dll.push_back(4);

        Box::new(dll)
    }

    #[test]
    fn pop() {
        let mut dll = generate_dll();

        assert_eq!(dll.pop_front(), Some(0));
        assert_eq!(dll.pop_front(), Some(1));
        assert_eq!(dll.pop_back(), Some(4));
        assert_eq!(dll.pop_back(), Some(3));
        assert_eq!(dll.pop_front(), Some(2));
        assert_eq!(dll.pop_front(), None);
    }

    #[test]
    fn remove() {
        let mut dll = generate_dll();

        assert_eq!(dll.remove(0), Some(0));
        assert_eq!(dll.remove(0), Some(1));
        assert_eq!(dll.remove(3), None);
        assert_eq!(dll.remove(2), Some(4));
        assert_eq!(dll.remove(1), Some(3));
        assert_eq!(dll.remove(0), Some(2));
        assert_eq!(dll.remove(0), None);
    }

    #[test]
    fn push() {
        let mut dll = Dll::default();
        dll.push_back(0);
        dll.push_back(1);
        dll.push_back(2);
        dll.push_front(3);
        dll.push_front(4);

        assert_eq!(dll.pop_front(), Some(4));
        assert_eq!(dll.pop_front(), Some(3));
        assert_eq!(dll.len(), 3);
        assert_eq!(dll.pop_back(), Some(2));
        assert_eq!(dll.pop_back(), Some(1));
        assert_eq!(dll.pop_back(), Some(0));
        assert_eq!(dll.pop_back(), None);
    }

    #[test]
    fn find() {
        let dll = generate_dll();
        for i in 0..dll.len() {
            let value = i as i32;
            let index = dll.find(&value);
            assert_eq!(index, Some(i));
        }

        for i in 0..dll.len() {
            let value = i as i32;
            let index = dll.find(&value);
            assert_eq!(index, Some(i));
        }
    }

    #[test]
    // fn iterate() {
    //     let dll = generate_dll();
    //     for (el, i) in dll {
    //         assert_eq!(i, 0);
    //     }
    // }
    #[test]
    fn clear_is_empty() {
        let mut dll = generate_dll();
        dll.clear();
        assert_eq!(dll.len(), 0);
        assert!(dll.is_empty());
    }

    #[test]
    fn len() {
        let mut dll = generate_dll();
        assert_eq!(dll.len(), 5);
        dll.clear();
        assert_eq!(dll.len(), 0);
        dll.push_back(1);
        dll.push_back(2);
        dll.push_back(3);
        dll.push_back(4);
        dll.push_back(5);
        assert_eq!(dll.len(), 5);
        dll.pop_back();
        assert_eq!(dll.len(), 4);
    }
}

mod tests_vec {
    use rust_group_project::{dll_trait::DoubleLinkedList, rc_refcell::Dll};

    /**
     * dll = [0, 1, 2, 3, 4] <=> [1, 2, 3] <=> [4, 5, 6] <=> [7, 8, 9] <=> [10, 11, 12]
     */
    fn generate_dll() -> Dll<Vec<i32>> {
        let mut dll = Dll::default();
        dll.push_back(vec![0, 1, 2, 3, 4]);
        dll.push_back(vec![1, 2, 3]);
        dll.push_back(vec![4, 5, 6]);
        dll.push_back(vec![7, 8, 9]);
        dll.push_back(vec![10, 11, 12]);

        dll
    }

    #[test]
    fn pop() {
        let mut dll = generate_dll();

        assert_eq!(dll.pop_front(), Some(vec![0, 1, 2, 3, 4]));
        assert_eq!(dll.pop_front(), Some(vec![1, 2, 3]));
        assert_eq!(dll.pop_back(), Some(vec![10, 11, 12]));
        assert_eq!(dll.pop_back(), Some(vec![7, 8, 9]));
        assert_eq!(dll.pop_front(), Some(vec![4, 5, 6]));
        assert_eq!(dll.pop_front(), None);
    }

    #[test]
    fn remove() {
        let mut dll = generate_dll();

        assert_eq!(dll.remove(0), Some(vec![0, 1, 2, 3, 4]));
        assert_eq!(dll.remove(0), Some(vec![1, 2, 3]));
        assert_eq!(dll.remove(3), None);
        assert_eq!(dll.remove(2), Some(vec![10, 11, 12]));
        assert_eq!(dll.remove(1), Some(vec![7, 8, 9]));
        assert_eq!(dll.remove(0), Some(vec![4, 5, 6]));
        assert_eq!(dll.remove(0), None);
    }

    #[test]
    fn push() {
        let mut dll = Dll::default();
        dll.push_back(vec![0, 1, 2, 3, 4]);
        dll.push_back(vec![1, 2, 3]);
        dll.push_back(vec![4, 5, 6]);
        dll.push_front(vec![7, 8, 9]);
        dll.push_front(vec![10, 11, 12]);

        assert_eq!(dll.pop_front(), Some(vec![10, 11, 12]));
        assert_eq!(dll.pop_front(), Some(vec![7, 8, 9]));
        assert_eq!(dll.len(), 3);
        assert_eq!(dll.pop_back(), Some(vec![4, 5, 6]));
        assert_eq!(dll.pop_back(), Some(vec![1, 2, 3]));
        assert_eq!(dll.pop_back(), Some(vec![0, 1, 2, 3, 4]));
        assert_eq!(dll.pop_back(), None);
    }

    #[test]
    fn find() {
        let mut dll = generate_dll();

        let data = dll.find(&vec![0, 1, 2, 3, 4]);
        assert_eq!(data, Some(0));
        let data = dll.find(&vec![1, 2, 3]);
        assert_eq!(data, Some(1));

        dll.push_front(vec![0, 1, 2, 3, 4]);
        let data = dll.find(&vec![0, 1, 2, 3, 4]);
        assert_eq!(data, Some(0));
    }

    #[test]
    fn clear_is_empty() {
        let mut dll = generate_dll();
        dll.clear();
        assert_eq!(dll.len(), 0);
        assert!(dll.is_empty());
    }

    #[test]
    fn len() {
        let mut dll = generate_dll();
        assert_eq!(dll.len(), 5);
        dll.clear();
        assert_eq!(dll.len(), 0);
        dll.push_back(vec![1, 2, 3]);
        dll.push_back(vec![4, 5, 6]);
        dll.push_back(vec![7, 8, 9]);
        dll.push_back(vec![10, 11, 12]);
        assert_eq!(dll.len(), 4);
        dll.pop_back();
        assert_eq!(dll.len(), 3);
    }

    #[test]
    fn get() {
        let mut dll = generate_dll();
        assert_eq!(dll.get(0), Some(&vec![0, 1, 2, 3, 4]));
        assert_eq!(dll.get(1), Some(&vec![1, 2, 3]));
        assert_eq!(dll.get(2), Some(&vec![4, 5, 6]));
        assert_eq!(dll.get(3), Some(&vec![7, 8, 9]));
        assert_eq!(dll.get(4), Some(&vec![10, 11, 12]));
        assert_eq!(dll.get(5), None);
    }
}
