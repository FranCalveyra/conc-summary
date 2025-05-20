use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod non_blocking_queue;
mod queue;
mod blocking_queue;

use crate::queue::Queue;
use non_blocking_queue::NonBlockingQueue;

fn main() {
    let queue = Arc::new(NonBlockingQueue::new());

    let threads: Vec<_> = (0..5)
        .map(|i| {
            let mut q = Arc::clone(&queue);
            thread::spawn(move || {
                for j in 0..10 {
                    q.enqueue(format!("Thread {i} - Item {j}"));
                    println!(
                        "[ENQUEUE] Thread {i} added item {j} | Size: {}",
                        q.size.load(Ordering::Relaxed)
                    );
                    thread::sleep(Duration::from_millis(10));
                }
            })
        })
        .collect();

    let dequeue_threads: Vec<_> = (0..5)
        .map(|i| {
            let mut q = Arc::clone(&queue);
            thread::spawn(move || {
                for _ in 0..10 {
                    if let Some(item) = q.dequeue() {
                        println!(
                            "[DEQUEUE] Thread {i} removed item: {item} | Size: {}",
                            q.size.load(Ordering::Relaxed)
                        );
                    } else {
                        println!("[DEQUEUE] Thread {i} tried to remove item, but queue was empty");
                    }
                    thread::sleep(Duration::from_millis(15));
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }
    for t in dequeue_threads {
        t.join().unwrap();
    }

    println!("Final size: {}", queue.size.load(Ordering::Relaxed));
}
