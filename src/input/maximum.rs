use input::Input;

use input::evaluator::InputEvaluatorRef;
use parser::{ Evaluator, Node };

// Maximum accumulator
// 
// Examines a bunch of inputs and takes the
// highest value.
pub struct Maximum {
  inputs : Vec<Box<Input>>,
}

impl Maximum {
  pub fn create(inputs_v: Vec<Box<Input>>) -> Box<Input> {
    Box::new(Maximum { inputs : inputs_v })
  }
  
  fn max(a: f64, b: f64) -> f64 {
    match a > b {
      true  => a,
      false => b,
    }
  }
}

impl Input for Maximum {
  fn compute(&mut self) -> f64 {
    let mut result : f64 = 0.0;
    for i in self.inputs.as_mut_slice() {
      result = Maximum::max(result, i.compute());
    }
    result
  }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////

pub struct EvalMaximum {
  input : InputEvaluatorRef,
}

impl EvalMaximum {
  pub fn new(input_v : InputEvaluatorRef) -> EvalMaximum {
    EvalMaximum { input : input_v }
  }
}

impl Evaluator<Box<Input>> for EvalMaximum {
  fn parse_nodes(&self, nodes: &[Node]) -> Result<Box<Input>, String> {
    Ok(Maximum::create(try!(self.input.borrow().parse_nodes(nodes))))
  }
}