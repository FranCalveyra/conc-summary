use crate::queue::Queue;
use std::ptr::{null, null_mut};
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

pub struct NonBlockingQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
    size: AtomicUsize,
}

impl<T> Queue<T> for NonBlockingQueue<T> {


    // How can I implement the dequeue operation?
    // [1 |]-> [2 |]-> [3 |]-> null
    // Head is 1, tail is 3
    // If I want to dequeue and remove the one, I must carry a copy of head
    fn dequeue(&mut self) -> Option<T> {
        None
    }


    fn enqueue(&mut self, item: T) {
        // Need to create a mutable pointer to a node
        // As the implementation is concurrent, node data must be in allocated in the heap
        // in order for other processes to complete unfinished operations
        let new_node = Box::into_raw(Box::new(Node::new(item)));

        // Variable for repeated ordering, still don't know how to use it
        let acquire = Ordering::Acquire;
        loop {
            let cur_tail = self.tail.load(acquire);
            let tail_next = unsafe { (*cur_tail).next.load(acquire) };
            if cur_tail == self.tail.load(acquire) {
                unsafe {
                    if !tail_next.is_null() {
                        self.tail
                            .compare_exchange(cur_tail, tail_next, acquire, acquire)
                            .unwrap();
                    } else if (*cur_tail).next.compare_exchange(
                        null_mut(), // Refers to a null pointer
                        new_node,
                        acquire,
                        acquire,
                    ) {
                        self.tail
                            .compare_exchange(cur_tail, new_node, acquire, acquire)
                            .expect("TODO: panic message");
                        return;
                    }
                }
            }
        }
    }
}

impl<T> NonBlockingQueue<T> {
    pub fn new() -> Self {
        // Create a dummy node on the heap
        let dummy = Box::into_raw(Box::new(Node::dummy()));
        NonBlockingQueue {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
            size: AtomicUsize::new(0),
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    pub item: Option<T>,
    pub next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    pub fn dummy() -> Self {
        Node {
            item: None,
            next: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    pub fn new(item: T) -> Self {
        Node {
            item: Some(item),
            next: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
}
