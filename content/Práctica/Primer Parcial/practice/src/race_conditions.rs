use std::sync::Arc;
use std::thread;

pub(crate) struct Counter {
    pub(crate) value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { value: 0 }
    }
    // Esto ni siquiera compila porque el borrow checker te protege
    pub fn increment(self: Arc<Self>){

        let mut local_counter = self.value.clone();
        println!("Current thread {} reads counter value as: {}", thread::current().name().unwrap(), local_counter);
        local_counter +=1;
        // self.value = local_counter;
        println!("Current thread {} now reads counter value as: {}", thread::current().name().unwrap(), self.value)
    }
}

