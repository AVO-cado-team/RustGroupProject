mod dll_trait;
mod rc_refcell;

use crate::dll_trait::DoubleLinkedList;
use crate::rc_refcell::Dll;
fn main() {
    let mut list = Dll::default();
    list.push_back(Box::new(1));
}
