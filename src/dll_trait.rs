pub trait DoubleLinkedList<T>
where
    T: Default + PartialEq,
{
    // fn default() -> Self;
    fn push_back(&mut self, value: T);
    fn push_front(&mut self, value: T);

    fn pop_back(&mut self) -> Option<T>;
    fn pop_front(&mut self) -> Option<T>;

    fn remove(&mut self, index: usize) -> Option<T>;
    fn find(&self, value: &T) -> Option<usize>;
    fn get(&self, index: usize) -> Option<&T>;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}
