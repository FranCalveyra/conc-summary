use std::env;
use std::sync::Arc;
use std::thread;

mod blocking_queue;
mod non_blocking_queue;
mod queue;

use crate::queue::Queue;
use non_blocking_queue::NonBlockingQueue;

fn main() {
    let args: Vec<String> = env::args().collect();

    let producers = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2);
    let consumers = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(2);
    let items = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(10);

    // println!("Producers: {}, Consumers: {}, Items: {}", producers, consumers, items); // Added for debugging

    let queue = Arc::new(NonBlockingQueue::new());

    let threads: Vec<_> = (0..producers)
        .map(|i| {
            let q = Arc::clone(&queue);
            thread::spawn(move || {
                for j in 0..items {
                    q.enqueue(format!("Thread {i} - Item {j}"));
                    // println!(
                    //     "[ENQUEUE] Producer {i} added item {j} | Size: {}",
                    //     q.size.load(Ordering::Relaxed)
                    // );
                    // thread::sleep(Duration::from_millis(10));
                }
            })
        })
        .collect();

    let dequeue_threads: Vec<_> = (0..consumers)
        .map(|_i| {
            let q = Arc::clone(&queue);
            thread::spawn(move || {
                for _ in 0..items {
                    if let Some(_item) = q.dequeue() {
                        // println!(
                        //     "[DEQUEUE] Consumer {i} removed item: {item} | Size: {}",
                        //     q.size.load(Ordering::Relaxed)
                        // );
                    } else {
                        // println!("[DEQUEUE] Consumer {i} tried to remove item, but queue was empty");
                    }
                    // thread::sleep(Duration::from_millis(15));
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

    // println!("Final size: {}", queue.size.load(Ordering::Relaxed));
}
