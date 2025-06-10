use crate::bounded_buffer::{BoundedBuffer, Consumer, Producer};
use crate::channels::pipeline;
use crate::philosophers::philosophers::{Philosopher, Table};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

mod bank_account;
mod bounded_buffer;
mod circular_buffer;
mod matrix;
mod merge_sort;
mod parallel_vector_sum;
mod philosophers;
mod queue;
mod race_conditions;
mod channels;

fn main() {
    pipeline()
}

fn bounded_buffer_main() {
    let buffer: BoundedBuffer<i32> = BoundedBuffer::new(5);
    let buffer_ref = Arc::new(buffer);
    let producers: Vec<Producer<i32>> = (0..5)
        .map(|i| Producer::new(buffer_ref.clone(), i))
        .collect();
    let consumers: Vec<Consumer<i32>> = (0..5)
        .map(|i| Consumer::new(buffer_ref.clone(), i))
        .collect();
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for (i, p) in producers.into_iter().enumerate() {
        let handle = thread::spawn(move || p.produce(i as i32));
        handles.push(handle);
    }
    for c in consumers {
        let handle = thread::spawn(move || {
            c.consume();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn race_conditions_main() {
    let counter = race_conditions::Counter::new();
    let counter_ref = Arc::new(counter);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| counter_ref.clone().increment());
        }
    })
}

fn philosophers_main() {
    let table = Arc::new(Table::new(5));
    let philosophers = (0..5).map(|i| Philosopher::new(i, table.clone()));

    let handles: Vec<JoinHandle<()>> = philosophers
        .map(|philosopher| thread::spawn(move || philosopher.eat()))
        .collect();

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
