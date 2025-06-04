use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub fn queue_behaviour() {
    let queue: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());
    thread::scope(|s| {
        s.spawn(|| {
            loop {
                // Busy loop !!
                let mut q = queue.lock().unwrap();
                if let Some(item) = q.pop_front() {
                    println!("Popped: {item}",);
                }
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            thread::sleep(Duration::from_secs(1));
        }
    })
}

pub fn queue_behaviour_with_condvar() {
    let queue: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();
    thread::scope(|s| {
        s.spawn(|| {
            loop {
                // Busy loop !!
                let mut q = queue.lock().unwrap();
                if let Some(item) = q.pop_front() {
                    println!("Popped: {item}",);
                }else{
                    not_empty.wait(q); // Wait
                }
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    })
}
