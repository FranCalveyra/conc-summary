use std::collections::VecDeque;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

trait Stack<T> {
    fn push(&self, value: T);
    fn pop(&self) -> Option<T>;
}

const ACQ: Ordering = Ordering::Acquire;
const REL: Ordering = Ordering::Release;

struct BlockingStack<T> {
    elements: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    not_full: Condvar,
    size: AtomicUsize,
    capacity: usize,
}

// impl<T> BlockingStack<T> {
//     pub fn new(capacity: usize) -> Self {
//         let vec = VecDeque::with_capacity(capacity);
//         let elements = Mutex::new(vec);
//         BlockingStack {
//             elements,
//             not_empty: Condvar::new(),
//             not_full: Condvar::new(),
//             size: AtomicUsize::new(0),
//             capacity,
//         }
//     }
// }
//
// impl<T> Stack<T> for BlockingStack<T> {
//     fn push(&self, value: T) {
//         let mut elements = self.elements.lock().unwrap();
//         while self.size.load(Ordering::Relaxed) >= self.capacity {
//             elements = self.not_full.wait(elements).unwrap();
//         }
//         elements.push_front(value);
//         self.size.fetch_add(1, Ordering::Relaxed);
//         self.not_empty.notify_one();
//     }
//
//     fn pop(&self) -> Option<T> {
//         let mut elements = self.elements.lock().unwrap();
//         while self.size.load(Ordering::Relaxed) == 0 {
//             elements = self.not_empty.wait(elements).unwrap();
//         }
//         let item = elements.pop_front();
//         self.size.fetch_sub(1, Ordering::Relaxed);
//         self.not_full.notify_one();
//         item
//     }
// }

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
        let new_node = Box::into_raw(Box::new(Node::new(value)));
        loop {
            let head = self.head.load(ACQ);
            unsafe { (*new_node).next.store(head, Ordering::Relaxed) };
            if self
                .head
                .compare_exchange(head, new_node, Ordering::Release, ACQ)
                .is_ok()
            {
                self.size.fetch_add(1, REL);
                break;
            }
        }
    }

    fn pop(&self) -> Option<T> {
        loop {
            let cur_head = self.head.load(ACQ);
            if cur_head.is_null() {
                return None;
            }
            let next_node = unsafe { (*cur_head).next.load(ACQ) };
            if self
                .head
                .compare_exchange(cur_head, next_node, Ordering::AcqRel, ACQ)
                .is_ok()
            {
                self.size.fetch_sub(1, REL);
                let old_head_node = unsafe { Box::from_raw(cur_head) };
                return old_head_node.item;
            }
        }
    }
}

struct Node<T> {
    item: Option<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn dummy() -> Self {
        Node {
            item: None,
            next: AtomicPtr::new(null_mut()),
        }
    }
    fn new(item: T) -> Self {
        Node {
            item: Some(item),
            next: AtomicPtr::new(null_mut()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    macro_rules! stack_tests {
        ($mod_name:ident, $stack_expr:expr) => {
            mod $mod_name {
                use super::*;
                #[test]
                fn single_thread_push_pop_simple() {
                    let stack = $stack_expr;
                    assert_eq!(stack.pop(), None);
                    stack.push(10);
                    stack.push(20);
                    assert_eq!(stack.pop(), Some(20));
                    assert_eq!(stack.pop(), Some(10));
                    assert_eq!(stack.pop(), None);
                }

                #[test]
                fn multi_thread_push_pop_stress() {
                    let stack = $stack_expr;
                    let results = Arc::new(Mutex::new(Vec::new()));
                    let threads: Vec<_> = (0..8)
                        .map(|t| {
                            let s = Arc::clone(&stack);
                            let r = Arc::clone(&results);
                            thread::spawn(move || {
                                let base = t * 1000;
                                for i in 0..1000 {
                                    s.push(base + i);
                                }
                                for _ in 0..1000 {
                                    if let Some(v) = s.pop() {
                                        r.lock().unwrap().push(v);
                                    }
                                }
                            })
                        })
                        .collect();
                    for th in threads {
                        th.join().unwrap();
                    }
                    assert_eq!(stack.pop(), None);
                    let vals = results.lock().unwrap();
                    assert_eq!(vals.len(), 8 * 1000);
                    let mut uniq = vals.clone();
                    uniq.sort_unstable();
                    uniq.dedup();
                    assert_eq!(uniq.len(), 8 * 1000);
                }

                #[test]
                fn concurrent_alternate_push_pop() {
                    let stack = $stack_expr;
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
                    assert_eq!(stack.pop(), None);
                }
            }
        };
    }

    stack_tests!(non_blocking, Arc::new(NonBlockingStack::new()));
    // stack_tests!(blocking, Arc::new(BlockingStack::new(1000)));
}
