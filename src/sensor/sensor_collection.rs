use crate::sensor::Sensor;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SensorCollection {
    sensors: Vec<Rc<RefCell<Box<dyn Sensor>>>>,
}

impl SensorCollection {
    // Creates empty sensor collection
    pub fn new() -> SensorCollection {
        SensorCollection {
            sensors: Vec::new(),
        }
    }

    // Adds a sensor to the collection
    pub fn add(&mut self, sensor: Rc<RefCell<Box<dyn Sensor>>>) {
        self.sensors.push(sensor);
    }

    // Updates all sensors
    pub fn update_all(&mut self) -> Result<(), String> {
        for s in self.sensors.as_mut_slice() {
            s.borrow_mut().update()?;
        }
        Ok(())
    }
}
