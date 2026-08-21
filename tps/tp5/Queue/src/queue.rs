pub trait Queue <T> {
    fn dequeue (&self) -> Option<T>;
    fn enqueue (&self, item: T);
}