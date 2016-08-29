use std::cell::RefCell;
use std::rc::Rc;

use fan::evaluator::NamedFans;
use input::evaluator::InputEvaluator;
use parser::{ Evaluator, Node };
use output::Output;
use util;

// Output evaluator
pub struct OutputEvaluator {
  named_fans      : Rc<RefCell<NamedFans>>,
  input_evaluator : Rc<RefCell<InputEvaluator>>,
}

impl OutputEvaluator {
  pub fn new(
    named_fans_v      : Rc<RefCell<NamedFans>>,
    input_evaluator_v : Rc<RefCell<InputEvaluator>>) -> OutputEvaluator {
    OutputEvaluator {
      named_fans      : named_fans_v,
      input_evaluator : input_evaluator_v }
  }
}

impl Evaluator<Output> for OutputEvaluator {
  fn parse_nodes(&self, nodes: &[Node]) -> Result<Output, String> {
    let fan_name   = try!(util::get_text_node("output", nodes, 0));
    let input_node = try!(util::get_node     ("output", nodes, 1));
    
    let mut named_fans  = self.named_fans.borrow_mut();
    let fan = try!(named_fans.remove(fan_name).ok_or(format!("No such sensor: {}", fan_name)));
    Ok(Output::new(fan, try!(self.input_evaluator.borrow().parse_node(input_node))))
  }
}