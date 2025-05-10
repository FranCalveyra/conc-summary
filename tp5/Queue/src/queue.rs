pub trait Queue <T> {
    fn dequeue (&mut self) -> Option<T>;
    fn enqueue (&mut self, item: T);
}