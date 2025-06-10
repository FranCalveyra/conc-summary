use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

struct BackoffCounter {
    value: AtomicUsize,
    sleep_time: AtomicUsize,
}

const ACQ: Ordering = Ordering::Acquire;
impl BackoffCounter {
    pub fn new(initial: usize) -> Self {
        let value = AtomicUsize::new(initial);
        BackoffCounter {
            value,
            sleep_time: AtomicUsize::new(1),
        }
    }

    pub fn increment(&self) {
        let val = self.value.load(Ordering::Acquire);
        let sleep = self.sleep_time.load(Ordering::Acquire);
        if self
            .value
            .compare_exchange_weak(val, val + 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            thread::sleep(Duration::from_micros(sleep as u64));
            if sleep < 128 {
                self.sleep_time.fetch_add(sleep, Ordering::AcqRel);
            }
        }
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Instant;

    struct NoBackoffCounter {
        value: AtomicUsize,
    }

    impl NoBackoffCounter {
        pub fn new(initial: usize) -> Self {
            NoBackoffCounter {
                value: AtomicUsize::new(initial),
            }
        }
        pub fn increment(&self) {
            let mut backoff = 1u64;
            loop {
                let cur = self.value.load(Ordering::Acquire);
                if self
                    .value
                    .compare_exchange_weak(cur, cur + 1, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {

                    break;
                }

                thread::sleep(Duration::from_micros(backoff));
                backoff = (backoff * 2).min(128);
            }
        }

        pub fn get(&self) -> usize {
            self.value.load(ACQ)
        }
    }

    // Hay muy pocos que fallan, me da 159903 en lugar de 16k
    #[test]
    fn backoff_vs_no_backoff_performance_and_correctness() {
        let threads = 16;
        let increments = 10_000;

        let backoff = Arc::new(BackoffCounter::new(0));
        let t0 = Instant::now();
        let handlers: Vec<_> = (0..threads)
            .map(|_| {
                let c = Arc::clone(&backoff);
                thread::spawn(move || {
                    for _ in 0..increments {
                        c.increment();
                    }
                })
            })
            .collect();
        for h in handlers {
            h.join().unwrap();
        }
        let dur_backoff = t0.elapsed();
        assert_eq!(backoff.get(), threads * increments);
        println!("tiempo con backoff: {:?}", dur_backoff);

        let noback = Arc::new(NoBackoffCounter::new(0));
        let t1 = Instant::now();
        let handlers: Vec<_> = (0..threads)
            .map(|_| {
                let c = Arc::clone(&noback);
                thread::spawn(move || {
                    for _ in 0..increments {
                        c.increment();
                    }
                })
            })
            .collect();
        for h in handlers {
            h.join().unwrap();
        }
        let dur_noback = t1.elapsed();
        assert_eq!(noback.get(), threads * increments);
        println!("tiempo sin backoff: {:?}", dur_noback);
    }
}
