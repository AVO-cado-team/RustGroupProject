use crate::dll_trait::DoubleLinkedList as DoubleLinkedListTrait;
use std::os::raw::c_int;

#[repr(C)]
pub struct DoubleLinkedListNode {
    data: c_int,
    next: *mut DoubleLinkedListNode,
    prev: *mut DoubleLinkedListNode,
}

#[repr(C)]
pub struct DoubleLinkedList {
    size: c_int,
    front: *mut DoubleLinkedListNode,
    list: *mut DoubleLinkedListNode,
    back: *mut DoubleLinkedListNode,
}

#[repr(C)]
pub struct DoubleLinkedListResult {
    data: c_int,
    result: c_int,
}

extern "C" {
    fn createDoubleLinkedList() -> *mut DoubleLinkedList;
    fn createDoubleLinkedListNode(data: c_int) -> *mut DoubleLinkedListNode;

    fn push_back(list: *mut DoubleLinkedList, data: c_int);
    fn push_front(list: *mut DoubleLinkedList, data: c_int);

    fn pop_back(list: *mut DoubleLinkedList) -> DoubleLinkedListResult;
    fn pop_front(list: *mut DoubleLinkedList) -> DoubleLinkedListResult;

    fn len(list: *mut DoubleLinkedList) -> c_int;
    fn clear(list: *mut DoubleLinkedList);
    fn is_empty(list: *mut DoubleLinkedList) -> c_int;

    fn find(list: *mut DoubleLinkedList, data: c_int) -> *mut DoubleLinkedListNode;
    fn get(list: *mut DoubleLinkedList, index: c_int) -> DoubleLinkedListResult;
    fn remove(list: *mut DoubleLinkedList, index: c_int) -> DoubleLinkedListResult;
}

pub struct Dll {
    list: *mut DoubleLinkedList,
}

impl Default for Dll {
    fn default() -> Dll {
        Dll {
            list: unsafe { createDoubleLinkedList() },
        }
    }
}

impl<T: Default + PartialEq + std::convert::From<i32> + Copy> DoubleLinkedListTrait<T> for Dll
where
    i32: std::convert::From<T>,
{
    fn push_back(&mut self, value: T) {
        unsafe {
            push_back(self.list, value.into());
        }
    }

    fn push_front(&mut self, value: T) {
        unsafe {
            push_front(self.list, value.into());
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        unsafe {
            let result = pop_back(self.list);
            if result.result == 0 {
                Some(result.data.into())
            } else {
                None
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        unsafe {
            let result = pop_front(self.list);
            if result.result == 0 {
                Some(result.data.into())
            } else {
                None
            }
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        unsafe {
            let result = remove(self.list, index as c_int);
            if result.result == 0 {
                Some(result.data.into())
            } else {
                None
            }
        }
    }

    fn find(&self, value: &T) -> Option<usize> {
        unsafe {
            let node = find(self.list, (*value).into());
            if node.is_null() {
                None
            } else {
                Some(node as usize)
            }
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        unsafe {
            let result = get(self.list, index as c_int);
            if result.result == 0 {
                Some(result.data.clone().into())
            } else {
                None
            }
        }
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }
}
