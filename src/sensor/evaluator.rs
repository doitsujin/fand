use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::parser::{Evaluator, Node, TagEvaluator};
use crate::sensor::sensor_hwmon::EvalHwmonSensor;
use crate::sensor::Sensor;
use crate::util;

pub type NamedSensors = HashMap<String, Rc<RefCell<Box<dyn Sensor>>>>;

// Sensor evaluator
//
// Evaluates an entry for a named sensor. Automatically
// creates the necessary tag evaluator during creation.
pub struct SensorEvaluator {
    tag_evaluator: TagEvaluator<Rc<RefCell<Box<dyn Sensor>>>>,
}

impl SensorEvaluator {
    pub fn new() -> SensorEvaluator {
        let mut tag_evaluator_v: TagEvaluator<Rc<RefCell<Box<dyn Sensor>>>> = TagEvaluator::new();
        tag_evaluator_v.add("hwmon-sensor", Rc::new(EvalHwmonSensor::new()));

        SensorEvaluator {
            tag_evaluator: tag_evaluator_v,
        }
    }
}

impl Evaluator<(String, Rc<RefCell<Box<dyn Sensor>>>)> for SensorEvaluator {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<(String, Rc<RefCell<Box<dyn Sensor>>>), String> {
        let name = util::get_text_node("sensor", nodes, 0)?;
        let node = util::get_node("sensor", nodes, 1)?;
        Ok((name.clone(), self.tag_evaluator.parse_node(node)?))
    }
}
