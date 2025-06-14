use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::leibniz::{calculate_leibniz_async, calculate_leibniz_thread};

pub trait Simulation {
    async fn simulate_io_tasks(&self,tasks: usize);
    async fn calculate_leibniz_term(&self,term_number:usize, divisions: usize);
    fn simulation_type(&self)-> SimulationType;
}

pub struct AsyncSimulation;
pub struct ThreadSimulation;
impl Simulation for AsyncSimulation  {
    async fn simulate_io_tasks(&self, tasks: usize) {
        let mut handles:Vec<tokio::task::JoinHandle<()>> = vec![];
        (0..tasks).for_each(|_| {
            let handle = tokio::spawn(async {
                tokio::time::sleep(Duration::from_millis(500)).await;
                println!("sleeping 500 milliseconds")
            });
            handles.push(handle);
        });

        for handle in handles{
            handle.await.unwrap_or_else(|_| { println!("Errorubi") })
        }
    }

    async fn calculate_leibniz_term(&self, term_number: usize, divisions: usize) {
        let term = calculate_leibniz_async(term_number, divisions);
        println!("Calculated result: {}", term.await)
    }
    fn simulation_type(&self) -> SimulationType { SimulationType::ASYNC }
}


impl Simulation for ThreadSimulation{
    async fn simulate_io_tasks(&self, tasks: usize){
        let mut handles: Vec<JoinHandle<()>> = vec![];
        (0..tasks).for_each(|_|{
            let handle = thread::spawn(||{
                thread::sleep(Duration::from_millis(500));
                println!("sleeping 500 milliseconds")
            });
            handles.push(handle);
        });
        for handle in handles{
            handle.join().unwrap()
        }
    }
    async fn calculate_leibniz_term(&self, term_number:usize, parts: usize){
        let term = calculate_leibniz_thread(term_number, parts);
        println!("Calculated result: {}", term)
    }
    fn simulation_type(&self) -> SimulationType { SimulationType::THREAD }
}

pub enum SimulationType{
    ASYNC,
    THREAD
}

impl SimulationType{
    pub fn from_string(string: String)-> Self{
        match &*string {
            "async"=> SimulationType::ASYNC,
            "thread"=>SimulationType::THREAD,
            _ => panic!()
        }
    }
}

