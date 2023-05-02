mod rc_refcell;
use rc_refcell::Dll;
use RustGroupProject::dll_trait::{self, DoubleLinkedList};
fn main() {
    let list = Dll::new(Some(1));
    println!("Hello, world!");
}
