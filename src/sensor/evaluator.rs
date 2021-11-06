use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::sensor::Sensor;
use crate::sensor::sensor_hwmon::EvalHwmonSensor;
use crate::parser::{ Evaluator, Node, TagEvaluator };
use crate::util;

pub type NamedSensors = HashMap<String, Rc<RefCell<Box<Sensor>>>>;

// Sensor evaluator
// 
// Evaluates an entry for a named sensor. Automatically
// creates the necessary tag evaluator during creation.
pub struct SensorEvaluator {
  tag_evaluator : TagEvaluator<Rc<RefCell<Box<Sensor>>>>,
}

impl SensorEvaluator {
  pub fn new() -> SensorEvaluator {
    let mut tag_evaluator_v : TagEvaluator<Rc<RefCell<Box<Sensor>>>> = TagEvaluator::new();
            tag_evaluator_v.add("hwmon-sensor", Rc::new(EvalHwmonSensor::new()));
    
    SensorEvaluator { tag_evaluator : tag_evaluator_v }
  }
}

impl Evaluator<(String, Rc<RefCell<Box<Sensor>>>)> for SensorEvaluator {
  fn parse_nodes(&self, nodes: &[Node]) -> Result<(String, Rc<RefCell<Box<Sensor>>>), String> {
    let name = r#try!(util::get_text_node("sensor", nodes, 0));
    let node = r#try!(util::get_node     ("sensor", nodes, 1));
    Ok((name.clone(), r#try!(self.tag_evaluator.parse_node(node))))
  }
}