// Descripción: Unos hilos producen datos y otros los consumen. El buffer tiene capacidad limitada.

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub struct BoundedBuffer<T> {
    data: Mutex<Data<T>>,
    not_empty: Condvar,
    not_full: Condvar,
}

struct Data<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    size: usize,
}

impl<T> Data<T> {
    pub fn new(capacity: usize) -> Self {
        Data {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            size: 0,
        }
    }
}

impl<T> BoundedBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        BoundedBuffer {
            data: Mutex::new(Data::new(capacity)),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }
}

pub struct Consumer<T> {
    buffer_ref: Arc<BoundedBuffer<T>>,
    consumer_id: i16,
}

impl<T> Consumer<T> {
    pub fn new(buffer_ref: Arc<BoundedBuffer<T>>, consumer_id: i16) -> Self {
        Consumer {
            buffer_ref,
            consumer_id,
        }
    }
    pub fn consume(&self) -> T {
        let mut data = self.buffer_ref.data.lock().unwrap();
        while data.size == 0 {
            println!(
                "Consumer with id {} is waiting, buffer is empty!",
                self.consumer_id
            );
            data = self.buffer_ref.not_empty.wait(data).unwrap();
        }
        let element = data.buffer.pop_front();
        data.size -= 1;
        drop(data);
        self.buffer_ref.not_full.notify_all();
        sleep(Duration::from_millis(500));
        println!("Consumer with id {} is consuming!", self.consumer_id);
        element.unwrap()
    }
}

pub struct Producer<T> {
    buffer_ref: Arc<BoundedBuffer<T>>,
    producer_id: i16,
}

impl<T> Producer<T> {
    pub fn new(buffer_ref: Arc<BoundedBuffer<T>>, producer_id: i16) -> Self {
        Producer {
            buffer_ref,
            producer_id,
        }
    }
    pub fn produce(&self, element: T) {
        let mut data = self.buffer_ref.data.lock().unwrap();
        while data.capacity == data.size {
            println!(
                "Producer with id {} is waiting, buffer is full!",
                self.producer_id
            );
            data = self.buffer_ref.not_full.wait(data).unwrap();
        }
        println!("Producer with id {} is producing!", self.producer_id);

        data.buffer.push_back(element);
        data.size += 1;
        sleep(Duration::from_millis(500));
        drop(data);

        self.buffer_ref.not_empty.notify_all();
        sleep(Duration::from_millis(500));
    }
}
