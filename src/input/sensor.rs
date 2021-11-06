use crate::input::Input;
use crate::sensor::Sensor;

use crate::parser::{Evaluator, Node};
use crate::sensor::evaluator::NamedSensors;
use crate::util;

use std::cell::RefCell;
use std::rc::Rc;

// Sensor input
pub struct SensorInput {
    sensor: Rc<RefCell<Box<dyn Sensor>>>,
}

impl SensorInput {
    pub fn create(sensor_v: Rc<RefCell<Box<dyn Sensor>>>) -> Box<dyn Input> {
        Box::new(SensorInput { sensor: sensor_v })
    }
}

impl Input for SensorInput {
    fn compute(&mut self) -> f64 {
        self.sensor.borrow().value()
    }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////

pub struct EvalSensorInput {
    named_sensors: Rc<RefCell<NamedSensors>>,
}

impl EvalSensorInput {
    pub fn new(named_sensors_v: Rc<RefCell<NamedSensors>>) -> EvalSensorInput {
        EvalSensorInput {
            named_sensors: named_sensors_v,
        }
    }
}

impl Evaluator<Box<dyn Input>> for EvalSensorInput {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<Box<dyn Input>, String> {
        let sensor_name = util::get_text_node("sensor-input", nodes, 0)?;
        let named_sensors = self.named_sensors.borrow();
        let sensor = named_sensors
            .get(sensor_name)
            .ok_or(format!("No such sensor: {}", sensor_name))?;
        Ok(SensorInput::create(sensor.clone()))
    }
}
