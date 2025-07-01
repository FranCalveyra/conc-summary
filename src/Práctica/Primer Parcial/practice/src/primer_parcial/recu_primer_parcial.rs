use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct BarberShop {
    capacity: usize,
    clients: Mutex<VecDeque<usize>>,
    can_cut: Condvar
}
impl BarberShop{
    fn new(capacity: usize)-> Self{
        let clients = Mutex::new(VecDeque::with_capacity(capacity));
        let can_cut = Condvar::new();
        BarberShop{ capacity, clients, can_cut }
    }

    fn enter(&self, client: usize)-> bool{
        let mut clients = self.clients.lock().unwrap();
        if clients.len() == self.capacity{
            return false;
        }
        clients.push_back(client);
        self.can_cut.notify_all();
        true
    }

    fn wait_for_turn_and_cut(&mut self, client: usize){
        let mut clients = self.clients.lock().unwrap();
        while *clients.front().unwrap() != client{
            clients = self.can_cut.wait(clients).unwrap();
        }
        clients.pop_front();
        drop(clients);
        self.can_cut.notify_all()
    }
}

fn main() {
    let shop = Arc::new(BarberShop::new(3));
    let mut handles = Vec::new();
    for i in 0..10 {
        let mut shop = Arc::clone(&shop);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 100));
            if shop.enter(i as usize){
                shop.wait_for_turn_and_cut(i as usize);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}