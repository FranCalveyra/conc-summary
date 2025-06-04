use std::sync::{Mutex, RwLock};

pub trait BankAccount {
    fn new(initial_balance: f64) -> Self;
    fn deposit(&self, amount: f64);
    fn withdraw(&self, amount: f64);
    fn get_balance(&self) -> f64;
}

pub struct MutexBankAccount {
    balance: Mutex<f64>,
}
impl BankAccount for MutexBankAccount {
    fn new(initial_balance: f64) -> MutexBankAccount {
        MutexBankAccount {
            balance: Mutex::new(initial_balance),
        }
    }
    fn deposit(&self, amount: f64) {
        if let Ok(mut balance) = self.balance.lock() {
            *balance += amount;
        }
    }
    fn withdraw(&self, amount: f64) {
        if let Ok(mut balance) = self.balance.lock() {
            if *balance < amount {
                println!("Insufficient balance!");
                return;
            }
            *balance -= amount;
        }
    }
    fn get_balance(&self) -> f64 {
        match self.balance.lock() {
            Ok(balance) => *balance,
            Err(_) => -1.0, // Negative value should indicate an error
        }
    }
}

struct RWBankAccount {
    balance: RwLock<f64>,
}

impl BankAccount for RWBankAccount {
    fn new(initial_balance: f64) -> Self {
        RWBankAccount {
            balance: RwLock::new(initial_balance),
        }
    }

    fn deposit(&self, amount: f64) {
        if let Ok(mut balance) = self.balance.write() {
            *balance += amount
        }
    }

    fn withdraw(&self, amount: f64) {
        if let Ok(mut balance) = self.balance.write() {
            if *balance < amount {
                println!("Insufficient amount");
                return;
            }
            *balance -= amount;
        }
    }

    fn get_balance(&self) -> f64 {
        match self.balance.read() {
            Ok(balance) => *balance,
            Err(_) => -1.0, // Negative value should indicate an error
        }
    }
}
