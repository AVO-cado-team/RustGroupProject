use crate::dll_trait::DoubleLinkedList;
use std::cell::UnsafeCell;
use std::rc::Rc;

type Link<T> = Rc<UnsafeCell<Node<T>>>;
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
    fn push_back(&mut self, value: T) {
        let new_node = Rc::new(UnsafeCell::new(Node::new(value, None, None)));
        match self.last.clone() {
            Some(last) => unsafe {
                (*last.get()).next = Some(new_node.clone());
                (*new_node.get()).prev = Some(last);
                self.last = Some(new_node);
            },
            None => {
                self.first = Some(new_node.clone());
                self.last = Some(new_node);
            }
        }
        self.length += 1;
    }

    fn push_front(&mut self, value: T) {
        let node = Rc::new(UnsafeCell::new(Node::new(value, None, None)));
        match self.first.clone() {
            Some(first) => unsafe {
                (*node.get()).next = Some(first.clone());
                (*first.get()).prev = Some(node.clone());
                self.first = Some(node);
            },
            None => {
                self.first = Some(node.clone());
                self.last = Some(node);
            }
        }
        self.length += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        let node = std::mem::replace(&mut self.last, None)?;
        unsafe {
            if let Some(prev) = (*node.get()).prev.clone() {
                (*prev.get()).next = None;
                self.last = Some(prev);
            } else {
                self.first = None;
                self.last = None;
            }
            self.length -= 1;
            Some(std::ptr::read(&(*node.get()).value))
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        let node = std::mem::replace(&mut self.first, None)?;
        unsafe {
            if let Some(next) = (*node.get()).next.clone() {
                (*next.get()).prev = None;
                self.first = Some(next);
            } else {
                self.last = None;
                self.first = None;
            }
            self.length -= 1;
            Some(std::ptr::read(&(*node.get()).value))
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            return self.pop_front();
        }
        if index == self.length - 1 {
            return self.pop_back();
        }

        let mut node = self.first.clone()?;
        for _i in 0..index {
            unsafe {
                node = (*node.get()).next.clone()?;
            }
        }

        unsafe {
            let prev = (*node.get()).prev.clone()?;
            let next = (*node.get()).next.clone()?;
            (*prev.get()).next = Some(next.clone());
            (*next.get()).prev = Some(prev);
            self.length -= 1;
            Some(std::ptr::read(&(*node.get()).value))
        }
    }

    fn find(&self, value: &T) -> Option<usize> {
        let mut node = self.first.clone()?;
        let mut index = 0;
        while unsafe { (*node.get()).value != *value } {
            node = unsafe { (*node.get()).next.clone()? };
            index += 1;
        }
        Some(index)
    }

    fn get(&self, index: usize) -> Option<&T> {
        let mut node = self.first.clone()?;
        for _i in 0..index {
            unsafe {
                node = (*node.get()).next.clone()?;
            }
        }
        unsafe { Some(&(*node.get()).value) }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn clear(&mut self) {
        let mut node = self.first.clone();

        while let Some(n) = node {
            unsafe {
                let next = (*n.get()).next.clone();
                drop(n.to_owned());
                node = next;
            }
        }

        self.first = None;
        self.last = None;
        self.length = 0;
    }
}
