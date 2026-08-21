use crate::queue::Queue;
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct BlockingQueue<T> {
    data: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    not_full: Condvar,
}

impl<T> Queue<T> for BlockingQueue<T> {
    fn dequeue(&self) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        while data.is_empty() {
            data = self.not_empty.wait(data).unwrap();
        }
        self.not_full.notify_all();
        data.pop_front()
    }

    fn enqueue(&self, item: T) {
        let mut data = self.data.lock().unwrap();
        while data.capacity() == data.len() {
            // Is full
            data = self.not_full.wait(data).unwrap();
        }
        self.not_empty.notify_all();
        data.push_back(item);
        drop(data);
    }
}

impl<T> BlockingQueue<T> {
    fn new(capacity: usize) -> Self {
        BlockingQueue {
            data: Mutex::new(VecDeque::with_capacity(capacity)),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocking_queue() {
        let queue = BlockingQueue::<i32>::new(10);
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }
}
