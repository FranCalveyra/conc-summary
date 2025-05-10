use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use crate::queue::Queue;

pub struct NonBlockingQueue<T>{
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
    dummy: Node<T>,
    size: AtomicUsize,
}

impl <T> Queue<T> for NonBlockingQueue<T>{

    fn dequeue (&mut self) -> Option<T>{

    }
    fn enqueue (&mut self, item: T){
        let mut new_node = Node::new(item);
        let new_node_ref = &mut new_node;
        loop {
            let cur_tail = &self.tail;
            let mut tail_next = self.tail.load(Ordering::Relaxed);
            if cur_tail == self.tail{
                if tail_next.is_some(){
                    self.tail.compare_exchange(cur_tail, tail_next, Ordering::Relaxed, Ordering::Relaxed)
                }
                else if let Ok(next_node) = cur_tail.load(Ordering::Relaxed).next{
                    if next_node.compare_exchange(None, new_node_ref,Ordering::Relaxed,Ordering::Relaxed){
                        self.tail.compare_exchange(cur_tail, new_node_ref, Ordering::Relaxed, Ordering::Relaxed).unwrap();
                        return
                    }
                }
            }
        }
    }

}

impl<T> NonBlockingQueue<T>{
    pub fn new()->Self{
        let mut dummy = Node::dummy();
        let dummy_ptr = &mut dummy;
        NonBlockingQueue{
            dummy,
            head: AtomicPtr::new(dummy_ptr),
            tail: AtomicPtr::new(dummy_ptr),
            size: AtomicUsize::new(0)

        }
    }
}

struct Node<T>{
    pub item: Option<T>,
    pub next: Option<AtomicPtr<Node<T>>>
}

impl<T> Node<T>{
    pub fn dummy()-> Self{
        Node{item: None, next: None}
    }

    pub fn new(item: T)-> Self{
        Node{item, next:None}
    }
}