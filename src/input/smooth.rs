use input::Input;

use input::evaluator::InputEvaluatorRef;
use parser::{ Evaluator, Node };
use util;

// Smoother
pub struct Smooth {
  input     : Box<Input>,
  values    : Vec<f64>,
  samples   : usize,
  index     : usize,
  
  curr_val  : f64,
  curr_sum  : f64,
}

impl Smooth {
  pub fn create(samples_v: usize, input_v: Box<Input>) -> Box<Input> {
    let mut values_v : Vec<f64> = Vec::new();
            values_v.resize(samples_v, 0.0);
    Box::new(Smooth {
      input     : input_v,
      values    : values_v,
      samples   : samples_v,
      index     : 0,
      
      curr_val  : 0.0,
      curr_sum  : 0.0,
    })
  }
}

impl Input for Smooth {
  fn compute(&mut self) -> f64 {
    let input = self.input.compute();
    
    // Compute sums without having to traverse the entire
    // buffer every time. For accuracy reasons, we compute
    // another sum from zero which will be swapped in after
    // each full round through the ring buffer.
    self.curr_val -= self.values[self.index];
    self.values[self.index] = input;
    
    self.curr_val += input;
    self.curr_sum += input;
    
    // Advance index and perform the magic explained
    // above to keep the numbers accurate enough.
    self.index += 1;
    if self.index == self.samples {
      self.curr_val = self.curr_sum;
      self.curr_sum = 0.0;
      self.index    = 0;
    }
    
    // Compute unweighed average
    self.curr_val / (self.samples as f64)
  }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////

pub struct EvalSmooth {
  input : InputEvaluatorRef,
}

impl EvalSmooth {
  pub fn new(input_v : InputEvaluatorRef) -> EvalSmooth {
    EvalSmooth { input : input_v }
  }
}

impl Evaluator<Box<Input>> for EvalSmooth {
  fn parse_nodes(&self, nodes: &[Node]) -> Result<Box<Input>, String> {
    Ok(Smooth::create(
      try!(util::get_num_node::<usize>("smooth", nodes, 0)),
      try!(self.input.borrow().parse_node(
        try!(util::get_node("smooth", nodes, 1))))))
  }
}