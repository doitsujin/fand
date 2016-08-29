use output::OutputCollection;
use sensor::SensorCollection;

use fan::evaluator::{ NamedFans, FanEvaluator };
use input::evaluator::InputEvaluator;
use output::evaluator::OutputEvaluator;
use sensor::evaluator::{ NamedSensors, SensorEvaluator };

use parser::{ Evaluator, Node };

use std::cell::RefCell;
use std::rc::Rc;

pub struct Config {
  pub sensors : SensorCollection,
  pub outputs : OutputCollection,
}

impl Config {
  
  pub fn new(s: SensorCollection, o: OutputCollection) -> Config {
    Config { sensors : s, outputs : o }
  }
  
  pub fn parse(nodes: &[Node]) -> Result<Config, String> {
    
    let named_fans      = Rc::new(RefCell::new(NamedFans::new()));
    let named_sensors   = Rc::new(RefCell::new(NamedSensors::new()));
    let input_evaluator = InputEvaluator::create(named_sensors.clone());
    
    let fan_eval    = FanEvaluator::new();
    let sensor_eval = SensorEvaluator::new();
    let output_eval = OutputEvaluator::new(named_fans.clone(), input_evaluator.clone());
    
    let mut sensors = SensorCollection::new();
    let mut outputs = OutputCollection::new();
    
    for n in nodes.iter() {
      match *n {
        Node::Node(ref s, ref nx) => match s.as_str() {
          "fan" => {
            let (name, fan) = try!(fan_eval.parse_nodes(nx));
            named_fans.borrow_mut().insert(name, fan);
          },
          "sensor" => {
            let (name, sensor) = try!(sensor_eval.parse_nodes(nx));
            named_sensors.borrow_mut().insert(name, sensor.clone());
            sensors.add(sensor);
          },
          "output" => {
            outputs.add(try!(output_eval.parse_nodes(nx)));
          },
          _ => { return Err(format!("Invalid root node type: {}", s)); }
        },
        Node::Text(ref s) => { return Err(format!("Expected node at '{}'", s)); }
      }
    }
    
    Ok(Config::new(sensors, outputs))
    
  }
  
}