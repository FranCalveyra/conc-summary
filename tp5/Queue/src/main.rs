use non_blocking_queue::NonBlockingQueue;

mod blocking_queue;
mod non_blocking_queue;
mod queue;

fn main() {
    let q: NonBlockingQueue<i32> = NonBlockingQueue::new();
}
