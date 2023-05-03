use crate::dll_trait::DoubleLinkedList;

pub struct Node<T> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T>
where
    T: Default + PartialEq,
{
    fn new(value: T, next: *mut Node<T>, prev: *mut Node<T>) -> Node<T> {
        Node { value, next, prev }
    }
    fn default() -> Node<T> {
        Node {
            value: T::default(),
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        }
    }
}

pub struct Dll<T> {
    first: *mut Node<T>,
    last: *mut Node<T>,
    length: usize,
}

impl<T: Default + PartialEq> DoubleLinkedList<T> for Dll<T> {
    fn push_back(&mut self, value: T) {
        let mut new_node = Node::new(value, std::ptr::null_mut(), std::ptr::null_mut());

        match self.last.is_null() {
            true => {
                self.first = &mut new_node;
                self.last = &mut new_node;
            }
            false => {
                unsafe {
                    (*self.last).next = &mut new_node;
                    new_node.prev = self.last;
                }
                self.last = &mut new_node;
            }
        }
        self.length += 1;
    }

    fn push_front(&mut self, value: T) {
        let mut new_node = Node::new(value, std::ptr::null_mut(), std::ptr::null_mut());

        match self.first.is_null() {
            true => {
                self.first = &mut new_node;
                self.last = &mut new_node;
            }
            false => {
                unsafe {
                    (*self.first).prev = &mut new_node;
                    new_node.next = self.first;
                }
                self.first = &mut new_node;
            }
        }
        self.length += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.last.is_null() {
            return None;
        }
        unsafe {
            let last = &mut *self.last;
            let value = std::ptr::read(&last.value);
            match last.prev.is_null() {
                true => {
                    self.first = std::ptr::null_mut();
                    self.last = std::ptr::null_mut();
                }
                false => {
                    self.last = last.prev;
                    (*self.last).next = std::ptr::null_mut();
                }
            }
            self.length -= 1;
            Some(value)
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.first.is_null() {
            return None;
        }
        unsafe {
            let first = &mut *self.first;
            let value = std::ptr::read(&first.value);
            match first.next.is_null() {
                true => {
                    self.first = std::ptr::null_mut();
                    self.last = std::ptr::null_mut();
                }
                false => {
                    self.first = first.next;
                    (*self.first).prev = std::ptr::null_mut();
                }
            }
            self.length -= 1;
            Some(value)
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if self.length == 0 || index >= self.length {
            return None;
        }
        unsafe {
            if index == 0 {
                return self.pop_front();
            } else if index == self.length - 1 {
                return self.pop_back();
            }
            let mut current = &mut *self.first;
            for _ in 0..index {
                current = &mut *current.next;
            }
            let value = std::ptr::read(&current.value);
            let next = &mut *current.next;
            let prev = &mut *current.prev;
            next.prev = current.prev;
            prev.next = current.next;
            self.length -= 1;
            Some(value)
        }
    }

    fn find(&self, value: &T) -> Option<usize> {
        if self.length == 0 {
            return None;
        }
        unsafe {
            let mut current = &mut *self.first;
            let mut index = 0;
            loop {
                if current.value == *value {
                    return Some(index);
                }
                if current.next.is_null() {
                    return None;
                }
                current = &mut *current.next;
                index += 1;
            }
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if self.length == 0 || index >= self.length {
            return None;
        }
        unsafe {
            let mut current = &mut *self.first;
            for _ in 0..index {
                current = &mut *current.next;
            }
            Some(&current.value)
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn clear(&mut self) {
        todo!()
    }
}
