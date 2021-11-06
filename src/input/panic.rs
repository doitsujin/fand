use crate::input::Input;

use crate::input::evaluator::InputEvaluatorRef;
use crate::parser::{Evaluator, Node};
use crate::util;

// Panic
//
// Ramps up a fan to full speed once a certain
// condition is met. Otherwise, returns zero so
// that this can be used with the accumulators.
pub struct Panic {
    input: Box<dyn Input>,
    temp_target: f64,
    temp_critical: f64,
    is_panicked: bool,
}

impl Panic {
    pub fn create(temp_target_v: f64, temp_critical_v: f64, input_v: Box<dyn Input>) -> Box<dyn Input> {
        Box::new(Panic {
            input: input_v,
            temp_target: temp_target_v,
            temp_critical: temp_critical_v,
            is_panicked: false,
        })
    }
}

impl Input for Panic {
    fn compute(&mut self) -> f64 {
        let input = self.input.compute();
        self.is_panicked = (!self.is_panicked && input >= self.temp_critical)
            || (self.is_panicked && input >= self.temp_target);
        match self.is_panicked {
            true => 1.0,
            false => 0.0,
        }
    }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////

pub struct EvalPanic {
    input: InputEvaluatorRef,
}

impl EvalPanic {
    pub fn new(input_v: InputEvaluatorRef) -> EvalPanic {
        EvalPanic { input: input_v }
    }
}

impl Evaluator<Box<dyn Input>> for EvalPanic {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<Box<dyn Input>, String> {
        Ok(Panic::create(
            util::get_num_node::<f64>("panic", nodes, 0)?,
            util::get_num_node::<f64>("panic", nodes, 1)?,
            self.input
                .borrow()
                .parse_node(util::get_node("panic", nodes, 2)?)?,
        ))
    }
}
