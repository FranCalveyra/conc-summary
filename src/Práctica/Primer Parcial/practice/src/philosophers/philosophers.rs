use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub struct Philosopher {
    table: Arc<Table>,
    left_fork: usize,
    right_fork: usize,
}

pub struct Table {
    pub forks: Mutex<Vec<bool>>,
    pub can_eat: Condvar,
}

impl Table {
    pub fn new(n: usize) -> Self {
        Table {
            forks: Mutex::new(vec![true; n]),
            can_eat: Condvar::new(),
        }
    }
}

impl Philosopher {
    pub fn new(right_fork: usize, table: Arc<Table>) -> Self {
        let left_fork = if right_fork == 0 {
            table.forks.lock().unwrap().len() - 1
        } else {
            right_fork - 1
        };
        Philosopher {
            left_fork,
            right_fork,
            table,
        }
    }

    pub fn eat(&self) {
        let mut forks = self.table.forks.lock().unwrap();

        while !(forks[self.left_fork] && forks[self.right_fork]) {
            forks = self.table.can_eat.wait(forks).unwrap();
        }
        // Acquire forks
        forks[self.left_fork] = false;
        forks[self.right_fork] = false;

        // Eat, emulate work
        println!("Philosopher {} is eating!", self.right_fork);
        sleep(Duration::from_millis(500));
        drop(forks);

        forks = self.table.forks.lock().unwrap();
        // Release forks
        forks[self.left_fork] = true;
        forks[self.right_fork] = true;

        // Notify forks were released
        drop(forks);
        self.table.can_eat.notify_all();
        println!("Philosopher {} is thinking!", self.right_fork);
        sleep(Duration::from_millis(500));
    }
}
