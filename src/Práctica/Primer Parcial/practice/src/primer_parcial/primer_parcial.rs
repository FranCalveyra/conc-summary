use std::cmp::Ordering;
use std::sync::{Condvar, Mutex};

pub struct Runway {
    id: u32,
    is_occupied: bool,
}

impl Runway {
    pub fn new(id: u32) -> Runway {
        Runway {
            id,
            is_occupied: false,
        }
    }
}

pub struct Plane {
    id: u32,
}
impl Plane {
    pub fn new(id: u32) -> Plane {
        Plane { id }
    }
}

pub struct Airport {
    runways: Mutex<Vec<Runway>>,
    can_land: Condvar,
}

impl Airport {
    pub fn new(runway_vec: Vec<Runway>) -> Airport {
        let runways = Mutex::new(runway_vec);
        let can_land = Condvar::new();
        Airport { runways, can_land }
    }
    pub fn request_runway(&self) -> u32 {
        let mut runways = self.runways.lock().unwrap();
        while runways.iter().all(|r| r.is_occupied) {
            runways = self.can_land.wait(runways).unwrap();
        }
        let free_runway: &Runway = runways.iter().find(|r| !r.is_occupied).unwrap();
        free_runway.id
    }

    pub fn release_runway(&self, runway_id: u32) {
        let mut runways = self.runways.lock().unwrap();
        let mut occupied_runway = runways.iter().find(|r| r.id == runway_id).unwrap();
        *occupied_runway.is_occupied = false;
        self.can_land.notify_one();
    }
}
