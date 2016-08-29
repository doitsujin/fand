use std::collections::HashMap;
use std::rc::Rc;

use fan::Fan;
use fan::fan_console::EvalConsoleFan;
use fan::fan_hwmon_pwm::EvalHwmonPwmFan;
use parser::{ Evaluator, Node, TagEvaluator };
use util;

pub type NamedFans = HashMap<String, Box<Fan>>;

// Fan evaluator
// 
// Evaluates an entry for a named fan. Automatically
// creates the necessary tag evaluator during creation.
pub struct FanEvaluator {
  tag_evaluator : TagEvaluator<Box<Fan>>,
}

impl FanEvaluator {
  pub fn new() -> FanEvaluator {
    let mut tag_evaluator_v : TagEvaluator<Box<Fan>> = TagEvaluator::new();
            tag_evaluator_v.add("console-fan", Rc::new(EvalConsoleFan::new()));
            tag_evaluator_v.add("hwmon-pwm",   Rc::new(EvalHwmonPwmFan::new()));
    
    FanEvaluator { tag_evaluator : tag_evaluator_v }
  }
}

impl Evaluator<(String, Box<Fan>)> for FanEvaluator {
  fn parse_nodes(&self, nodes: &[Node]) -> Result<(String, Box<Fan>), String> {
    let name = try!(util::get_text_node("fan", nodes, 0));
    let node = try!(util::get_node     ("fan", nodes, 1));
    Ok((name.clone(), try!(self.tag_evaluator.parse_node(node))))
  }
}