use std::collections::VecDeque;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex};

trait Stack<T> {
    fn push(&self, value: T);
    fn pop(&self) -> Option<T>;
}

struct BlockingStack<T> {
    elements: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    not_full: Condvar,
    size: AtomicUsize,
    capacity: usize,
}

// Same as the Blocking queue
impl<T> BlockingStack<T> {
    pub fn new(capacity: usize) -> Self {
        let vec: VecDeque<T> = VecDeque::with_capacity(capacity);
        let size = AtomicUsize::new(0);
        let elements = Mutex::new(vec);
        BlockingStack {
            elements,
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            size,
            capacity,
        }
    }
}

impl<T> Stack<T> for BlockingStack<T> {
    fn push(&self, value: T) {
        let mut elements = self.elements.lock().unwrap();
        // While is full
        while self.size.load(Ordering::Relaxed) >= self.capacity {
            elements = self.not_empty.wait(elements).unwrap()
        }
        elements.push_front(value);
        self.size.fetch_add(1, Ordering::Relaxed);
        self.not_empty.notify_all()
    }

    fn pop(&self) -> Option<T> {
        let mut elements = self.elements.lock().unwrap();
        // While is empty
        while self.size.load(Ordering::Relaxed) == self.capacity {
            elements = self.not_full.wait(elements).unwrap()
        }
        let element = elements.pop_front();
        self.not_full.notify_all();
        self.size.fetch_sub(1, Ordering::Relaxed);
        element
    }
}

struct NonBlockingStack<T> {
    head: AtomicPtr<Node<T>>,
    size: AtomicUsize,
}

impl<T> NonBlockingStack<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node::dummy()));
        NonBlockingStack {
            head: AtomicPtr::new(dummy),
            size: AtomicUsize::new(0),
        }
    }
}

impl<T> Stack<T> for NonBlockingStack<T> {
    fn push(&self, value: T) {
        let acquire = Ordering::Acquire;
        let release = Ordering::Release;
        let new_head = Box::into_raw(Box::new(Node::new(value)));
        loop {
            // Get current head
            let cur_head = self.head.load(acquire);
            // Update the next node's next (intermediate step of assigning it the old head as its next)
            unsafe {
                (*new_head).next = AtomicPtr::from(cur_head);
            }
            // Finish operation (both other and own)
            if self
                .head
                .compare_exchange(cur_head, new_head, release, acquire)
                .is_ok()
            {
                // Add one to size
                self.size.fetch_add(1, Ordering::Release);
                break;
            }
        }
    }

    // It's actually the same as the dequeue from the TP5
    fn pop(&self) -> Option<T> {
        let acquire = Ordering::Acquire;
        loop {
            // Try to acquire head
            let mut cur_head = unsafe { self.head.load(acquire) };

            if cur_head.is_null() {
                return None; // Empty stack case
            }

            let next = unsafe { (*cur_head).next.load(acquire) };

            // If already been popped (completing operation for other one)
            if self
                .head
                .compare_exchange(cur_head, next, Ordering::AcqRel, acquire)
                .is_ok()
            {
                self.size.fetch_sub(1, Ordering::Release);
                let old_head_node = unsafe { Box::from_raw(cur_head) };
                return old_head_node.item;
            }
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
            next: AtomicPtr::new(null_mut()),
        }
    }

    pub fn new(item: T) -> Self {
        Node {
            item: Some(item),
            next: AtomicPtr::new(null_mut()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // import NonBlockingStack
        use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn single_thread_push_pop_simple() {
        let stack = NonBlockingStack::new();
        // empty at start
        assert_eq!(stack.pop(), None);

        // push two values
        stack.push(10);
        stack.push(20);

        // pop in LIFO order
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));

        // now empty again
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn multi_thread_push_pop_stress() {
        let stack = Arc::new(NonBlockingStack::new());
        let results = Arc::new(Mutex::new(Vec::new()));

        let threads: Vec<_> = (0..8)
            .map(|t| {
                let stack = Arc::clone(&stack);
                let results = Arc::clone(&results);
                thread::spawn(move || {
                    // each thread pushes 1_000 unique values
                    let base = t * 1_000;
                    for i in 0..1_000 {
                        stack.push(base + i);
                    }
                    // then pops 1_000 values
                    for _ in 0..1_000 {
                        if let Some(v) = stack.pop() {
                            results.lock().unwrap().push(v);
                        }
                    }
                })
            })
            .collect();

        // wait for all threads to finish
        for th in threads {
            th.join().unwrap();
        }

        // stack should now be empty
        assert_eq!(stack.pop(), None);

        // collect all popped values
        let vals = results.lock().unwrap();
        assert_eq!(vals.len(), 8 * 1_000);

        // check for no duplicates / missing values
        let mut uniq = vals.clone();
        uniq.sort_unstable();
        uniq.dedup();
        assert_eq!(uniq.len(), 8 * 1_000);
    }
}

#[cfg(test)]
mod non_blocking_tests {
    use super::{NonBlockingStack, Stack};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn empty_pop_returns_none() {
        let stack: NonBlockingStack<i32> = NonBlockingStack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn single_thread_push_pop_alternate() {
        let stack = NonBlockingStack::new();
        for i in 0..10 {
            stack.push(i);
            assert_eq!(stack.pop(), Some(i));
        }
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn concurrent_alternate_push_pop() {
        let stack = Arc::new(NonBlockingStack::new());
        let handles: Vec<_> = (0..4)
            .map(|t| {
                let s = Arc::clone(&stack);
                thread::spawn(move || {
                    for i in 0..500 {
                        s.push(t * 500 + i);
                        thread::sleep(Duration::from_micros(10));
                        assert!(s.pop().is_some());
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        // after all alternations, stack should be empty
        assert_eq!(stack.pop(), None);
    }
}

#[cfg(test)]
mod blocking_tests {
    use super::{BlockingStack, Stack};
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread,
        time::Duration,
    };

    #[test]
    fn single_thread_push_pop_simple() {
        let stack = BlockingStack::new(2);
        assert_eq!(stack.pop(), None);

        stack.push(100);
        stack.push(200);
        assert_eq!(stack.pop(), Some(200));
        assert_eq!(stack.pop(), Some(100));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn blocks_when_full_and_unblocks_on_pop() {
        let stack = Arc::new(BlockingStack::new(1));
        stack.push(1);

        let (tx, rx) = mpsc::channel();
        let s2 = Arc::clone(&stack);

        // this thread will block on push(2) until main thread pops
        thread::spawn(move || {
            s2.push(2);
            tx.send(()).unwrap();
        });

        // give the spawned thread time to attempt the push
        thread::sleep(Duration::from_millis(50));
        // it should still be blocked, so no message yet
        assert!(rx.recv_timeout(Duration::from_millis(10)).is_err());

        // unblock by popping
        assert_eq!(stack.pop(), Some(1));

        // now the push(2) should complete
        assert!(rx.recv_timeout(Duration::from_secs(1)).is_ok());
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn multi_thread_push_pop_stress() {
        let stack = Arc::new(BlockingStack::new(1000));
        let results = Arc::new(Mutex::new(Vec::new()));

        let threads: Vec<_> = (0..8)
            .map(|t| {
                let s = Arc::clone(&stack);
                let res = Arc::clone(&results);
                thread::spawn(move || {
                    let base = t * 1_000;
                    for i in 0..1_000 {
                        s.push(base + i);
                    }
                    for _ in 0..1_000 {
                        if let Some(v) = s.pop() {
                            res.lock().unwrap().push(v);
                        }
                    }
                })
            })
            .collect();

        for h in threads {
            h.join().unwrap();
        }

        assert_eq!(stack.pop(), None);

        let vals = results.lock().unwrap();
        assert_eq!(vals.len(), 8 * 1_000);

        let mut uniq = vals.clone();
        uniq.sort_unstable();
        uniq.dedup();
        assert_eq!(uniq.len(), 8 * 1_000);
    }
}
