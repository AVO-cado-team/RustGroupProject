use rstest_reuse::{self, *};

mod tests_i32 {
    use rstest::{fixture, rstest};
    use rstest_reuse::{self, *};
    use rust_group_project::{
        dll_trait::DoubleLinkedList, r#unsafe::Dll as UnsafeDll, r#unsafe::Dll as UnsafeCellDll,
        rc_refcell::Dll as RCDll,
    };

    #[fixture]
    fn generate_rcdll() -> Box<dyn DoubleLinkedList<i32>> {
        let mut dll = RCDll::default();
        dll.push_back(0);
        dll.push_back(1);
        dll.push_back(2);
        dll.push_back(3);
        dll.push_back(4);
        Box::new(dll)
    }

    #[fixture]
    fn generate_unsafedll() -> Box<dyn DoubleLinkedList<i32>> {
        let mut dll = UnsafeCellDll::default();
        dll.push_back(0);
        dll.push_back(1);
        dll.push_back(2);
        dll.push_back(3);
        dll.push_back(4);
        Box::new(dll)
    }

    #[template]
    #[rstest]
    fn base(
        #[values(generate_rcdll(), generate_unsafedll())] mut dll: Box<dyn DoubleLinkedList<i32>>,
    ) {
    }

    #[apply(base)]
    fn pop(mut dll: Box<dyn DoubleLinkedList<i32>>) {
        assert_eq!(dll.pop_front(), Some(0));
        assert_eq!(dll.pop_front(), Some(1));
        assert_eq!(dll.pop_back(), Some(4));
        assert_eq!(dll.pop_back(), Some(3));
        assert_eq!(dll.pop_front(), Some(2));
        assert_eq!(dll.pop_front(), None);
    }

    #[apply(base)]
    fn remove(mut dll: Box<dyn DoubleLinkedList<i32>>) {
        assert_eq!(dll.remove(0), Some(0));
        assert_eq!(dll.remove(0), Some(1));
        assert_eq!(dll.remove(3), None);
        assert_eq!(dll.remove(2), Some(4));
        assert_eq!(dll.remove(1), Some(3));
        assert_eq!(dll.remove(0), Some(2));
        assert_eq!(dll.remove(0), None);
    }

    #[apply(base)]
    fn push(mut dll: Box<dyn DoubleLinkedList<i32>>) {
        assert_eq!(dll.pop_front(), Some(0));
        assert_eq!(dll.pop_front(), Some(1));
        assert_eq!(dll.len(), 3);
        assert_eq!(dll.pop_back(), Some(4));
        assert_eq!(dll.pop_back(), Some(3));
        assert_eq!(dll.pop_back(), Some(2));
        assert_eq!(dll.pop_back(), None);
    }

    #[apply(base)]
    fn find(dll: Box<dyn DoubleLinkedList<i32>>) {
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

    #[apply(base)]
    fn clear_is_empty(mut dll: Box<dyn DoubleLinkedList<i32>>) {
        dll.clear();
        assert_eq!(dll.len(), 0);
        assert!(dll.is_empty());
    }

    #[apply(base)]
    fn len(mut dll: Box<dyn DoubleLinkedList<i32>>) {
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
    use rstest::{fixture, rstest};
    use rust_group_project::{
        dll_trait::DoubleLinkedList, r#unsafe::Dll as UnsafeDll, r#unsafe::Dll as UnsafeCellDll,
        rc_refcell::Dll as RCDll,
    };

    #[fixture]
    fn generate_rcdll() -> Box<dyn DoubleLinkedList<Vec<i32>>> {
        let mut dll = RCDll::default();
        dll.push_back(vec![0, 1, 2, 3, 4]);
        dll.push_back(vec![1, 2, 3]);
        dll.push_back(vec![4, 5, 6]);
        dll.push_back(vec![7, 8, 9]);
        dll.push_back(vec![10, 11, 12]);

        Box::new(dll)
    }

    #[fixture]
    fn generate_unsafedll() -> Box<dyn DoubleLinkedList<Vec<i32>>> {
        let mut dll = UnsafeCellDll::default();
        dll.push_back(vec![0, 1, 2, 3, 4]);
        dll.push_back(vec![1, 2, 3]);
        dll.push_back(vec![4, 5, 6]);
        dll.push_back(vec![7, 8, 9]);
        dll.push_back(vec![10, 11, 12]);

        Box::new(dll)
    }

    #[rstest]
    fn pop(
        #[values(generate_rcdll(), generate_unsafedll())] mut dll: Box<
            dyn DoubleLinkedList<Vec<i32>>,
        >,
    ) {
        assert_eq!(dll.pop_front(), Some(vec![0, 1, 2, 3, 4]));
        assert_eq!(dll.pop_front(), Some(vec![1, 2, 3]));
        assert_eq!(dll.pop_back(), Some(vec![10, 11, 12]));
        assert_eq!(dll.pop_back(), Some(vec![7, 8, 9]));
        assert_eq!(dll.pop_front(), Some(vec![4, 5, 6]));
        assert_eq!(dll.pop_front(), None);
    }

    #[rstest]
    fn remove(
        #[values(generate_rcdll(), generate_unsafedll())] mut dll: Box<
            dyn DoubleLinkedList<Vec<i32>>,
        >,
    ) {
        assert_eq!(dll.remove(0), Some(vec![0, 1, 2, 3, 4]));
        assert_eq!(dll.remove(0), Some(vec![1, 2, 3]));
        assert_eq!(dll.remove(3), None);
        assert_eq!(dll.remove(2), Some(vec![10, 11, 12]));
        assert_eq!(dll.remove(1), Some(vec![7, 8, 9]));
        assert_eq!(dll.remove(0), Some(vec![4, 5, 6]));
        assert_eq!(dll.remove(0), None);
    }

    #[rstest]
    fn push(
        #[values(generate_rcdll(), generate_unsafedll())] mut dll: Box<
            dyn DoubleLinkedList<Vec<i32>>,
        >,
    ) {
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

    #[rstest]
    fn find(
        #[values(generate_rcdll(), generate_unsafedll())] mut dll: Box<
            dyn DoubleLinkedList<Vec<i32>>,
        >,
    ) {
        let data = dll.find(&vec![0, 1, 2, 3, 4]);
        assert_eq!(data, Some(0));
        let data = dll.find(&vec![1, 2, 3]);
        assert_eq!(data, Some(1));

        dll.push_front(vec![0, 1, 2, 3, 4]);
        let data = dll.find(&vec![0, 1, 2, 3, 4]);
        assert_eq!(data, Some(0));
    }

    #[rstest]
    fn clear(
        #[values(generate_rcdll(), generate_unsafedll())] mut dll: Box<
            dyn DoubleLinkedList<Vec<i32>>,
        >,
    ) {
        dll.clear();
        assert_eq!(dll.len(), 0);
        assert!(dll.is_empty());
    }

    #[rstest]
    fn len(
        #[values(generate_rcdll(), generate_unsafedll())] mut dll: Box<
            dyn DoubleLinkedList<Vec<i32>>,
        >,
    ) {
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

    #[rstest]
    fn get(
        #[values(generate_rcdll(), generate_unsafedll())] dll: Box<dyn DoubleLinkedList<Vec<i32>>>,
    ) {
        assert_eq!(dll.get(0), Some(&vec![0, 1, 2, 3, 4]));
        assert_eq!(dll.get(1), Some(&vec![1, 2, 3]));
        assert_eq!(dll.get(2), Some(&vec![4, 5, 6]));
        assert_eq!(dll.get(3), Some(&vec![7, 8, 9]));
        assert_eq!(dll.get(4), Some(&vec![10, 11, 12]));
        assert_eq!(dll.get(5), None);
    }
}
