use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use crate::queue::Queue;

pub struct BlockingQueue<T> {
    data: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    not_full: Condvar,
}

impl <T> Queue<T> for BlockingQueue<T> {

    fn dequeue(&mut self) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        while data.is_empty(){
            data = self.not_empty.wait(data).unwrap();
        }
        self.not_full.notify_all();
        data.pop_front()
    }

    fn enqueue(&mut self, item: T) {
        let mut data = self.data.lock().unwrap();
        while data.capacity() == data.len() { // Is full
            data = self.not_full.wait(data).unwrap();
        }
        self.not_empty.notify_all();
        data.push_back(item);
        drop(data);
    }
}

impl <T> BlockingQueue<T>{
    fn new (capacity: usize) -> Self {
        BlockingQueue {
            data: Mutex::new(VecDeque::with_capacity(capacity)),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }
}