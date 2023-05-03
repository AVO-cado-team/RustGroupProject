use crate::dll_trait::DoubleLinkedList;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;
#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Link<T>>,
    prev: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Link<T>>, prev: Option<Link<T>>) -> Node<T> {
        Node { value, next, prev }
    }
}

#[derive(Default)]
pub struct Dll<T> {
    first: Option<Link<T>>,
    last: Option<Link<T>>,
    length: usize,
}

impl<T: Default + PartialEq> DoubleLinkedList<T> for Dll<T> {
    fn new(value: T) -> Self {
        let node = Rc::new(RefCell::new(Node::new(value, None, None)));
        let first = Some(node.clone());
        let last = Some(node.clone());
        Dll {
            first,
            last,
            length: 1,
        }
    }

    fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value, None, None)));
        match self.last.clone() {
            Some(last) => {
                (*new_node).borrow_mut().prev = Some(last.clone());
                (*last).borrow_mut().next = Some(new_node.clone());
                self.last = Some(new_node);
            }
            None => {
                self.first = Some(new_node.clone());
                self.last = Some(new_node.clone());
            }
        }
        self.length += 1;
    }

    fn push_forward(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value, None, None)));
        match self.first.clone() {
            Some(first) => {
                (*node).borrow_mut().next = Some(first.clone());
                (*first).borrow_mut().prev = Some(node.clone());
                self.first = Some(node);
            }
            None => {
                self.first = Some(node.clone());
                self.last = Some(node);
            }
        }
        self.length += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        let node = std::mem::replace(&mut self.last, None)?;
        if let Some(prev) = (*node).borrow_mut().prev.as_mut() {
            (*prev).borrow_mut().next = None;
        } else {
            self.first = None;
        }

        let node = Rc::try_unwrap(node)
            .ok()
            .expect("We have other references to the last node");
        let node = node.into_inner();
        self.last = node.prev;
        self.length -= 1;
        Some(node.value)
    }

    fn pop_forward(&mut self) -> Option<T> {
        let node = std::mem::replace(&mut self.first, None)?;
        if let Some(next) = (*node).borrow_mut().next.as_mut() {
            (*next).borrow_mut().prev = None;
        } else {
            self.first = None;
            self.last = None;
        }

        let node = Rc::try_unwrap(node)
            .ok()
            .expect("We have other references to the first node");
        let node = node.into_inner();
        self.first = node.next;
        self.length -= 1;
        Some(node.value)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        } else if index == 0 {
            return self.pop_forward();
        } else if index == self.length - 1 {
            return self.pop_back();
        }

        let mut node = self.first.clone();
        for _ in 0..index {
            node = node?.borrow().next.clone();
        }

        let node = node.unwrap();
        let mut node = node.borrow_mut();

        let binding = node.prev.clone().unwrap();
        let mut prev = binding.borrow_mut();
        let binding = node.next.clone().unwrap();
        let mut next = binding.borrow_mut();
        prev.next = node.next.clone();
        next.prev = node.prev.clone();

        node.prev = None;
        node.next = None;

        self.length -= 1;
        Some(std::mem::take(&mut node.value))
    }

    fn find(&self, value: &T) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut index = 0;
        let mut current_node = self.first.clone();
        loop {
            let binding = current_node.clone()?;
            let node_borrow = binding.borrow();
            current_node = binding.borrow().next.clone();
            let node_borrow = &*node_borrow;
            let node_value = unsafe { &*(&node_borrow.value as *const T) };

            if node_value == value {
                return Some(index);
            }

            index += 1;
        }
    }

    /**
     * Returns the element at the given index.
     */
    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }

        let mut node = self.first.clone();
        for _ in 0..index {
            node = node?.borrow().next.clone();
        }

        let node = node?;
        let node = &*node.borrow();
        // SAFETY: ...
        let value = unsafe { &*(&node.value as *const T) };
        Some(value)
    }

    fn len(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.first.is_none() && self.last.is_none()
    }

    fn clear(&mut self) {
        // Pop all elements from the list and free them
        while self.pop_back().is_some() {}
    }
}

pub struct DllIterator<T> {
    dll: Dll<T>,
}

impl<T> Iterator for DllIterator<T>
where
    T: Default + PartialEq,
    Dll<T>: DoubleLinkedList<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        return self.dll.pop_forward();
    }
}

impl<T> IntoIterator for Dll<T>
where
    T: Default + PartialEq,
{
    // Implement IntoIterator trait for Dll
    type Item = T;
    type IntoIter = DllIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        DllIterator { dll: self }
    }
}

mod tests {
    use super::*;

    fn generate_dll() -> Dll<i32> {
        let mut dll = Dll::new(0);
        dll.push_back(1);
        dll.push_back(2);
        dll.push_back(3);
        dll.push_back(4);

        dll
    }
    #[test]
    fn test_new() {
        let dll = Dll::new(1);
        assert_eq!(dll.len(), 1);
        assert_eq!(dll.get(0), Some(&1));
    }

    #[test]
    fn test_pop() {
        let mut dll = Dll::new(0);
        dll.push_back(1);
        dll.push_back(2);

        let last_element = dll.pop_back().unwrap();
        assert_eq!(last_element, 2);
    }

    #[test]
    fn test_remove() {
        // dll = [0, 1, 2]
        let mut dll = Dll::new(0);
        dll.push_back(1);
        dll.push_back(2);

        let index = 1;
        let data = dll.get(index).unwrap();
        let _removed_data = dll.remove(index).unwrap();
        let removed_data = dll.remove(index).unwrap();
        // dll = [0, 2]
        assert_eq!(2, removed_data);
    }

    #[test]
    fn test_push_back() {
        let mut dll = Dll::new(1);
        dll.push_back(2);
        assert_eq!(dll.len(), 2);
        assert_eq!(dll.get(0), Some(&1));
        assert_eq!(dll.get(1), Some(&2));
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
    fn iterate_and_print() {
        let mut dll = Dll::new(0);
        dll.push_back(1);
        dll.push_back(2);
        dll.push_back(3);
        dll.push_back(4);
        for i in dll {
            println!("{}", i);
        }
        // print!("{:?}", dll.pop_back());
    }
}
