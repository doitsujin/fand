use crate::input::Input;

use crate::input::evaluator::InputEvaluatorRef;
use crate::parser::{Evaluator, Node};
use crate::util;

// Cutoff
//
// Stops fans when the target speed is too low,
// and starts the fans once the target speed
// exceeds a given value.
pub struct Cutoff {
    input: Box<dyn Input>,
    stop_below: f64,
    start_above: f64,
    start_value: f64,
    is_stopped: bool,
}

impl Cutoff {
    pub fn create(
        stop_below_v: f64,
        start_above_v: f64,
        start_value_v: f64,
        input_v: Box<dyn Input>,
    ) -> Box<dyn Input> {
        Box::new(Cutoff {
            input: input_v,
            stop_below: stop_below_v,
            start_above: start_above_v,
            start_value: start_value_v,
            is_stopped: true,
        })
    }
}

impl Input for Cutoff {
    fn compute(&mut self) -> f64 {
        let input = self.input.compute();

        if self.is_stopped {
            // Start fans if necessary
            if input >= self.start_above {
                self.is_stopped = false;
                self.start_value
            } else {
                0.0
            }
        } else if input < self.stop_below {
            // Stop fans if the input value dropped below the
            // value where the fans shall be switched off
            self.is_stopped = true;
            0.0
        } else {
            // Use raw input
            input
        }
    }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////

pub struct EvalCutoff {
    input: InputEvaluatorRef,
}

impl EvalCutoff {
    pub fn new(input_v: InputEvaluatorRef) -> EvalCutoff {
        EvalCutoff { input: input_v }
    }
}

impl Evaluator<Box<dyn Input>> for EvalCutoff {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<Box<dyn Input>, String> {
        Ok(Cutoff::create(
            util::get_num_node::<f64>("cutoff", nodes, 0)?,
            util::get_num_node::<f64>("cutoff", nodes, 1)?,
            util::get_num_node::<f64>("cutoff", nodes, 2)?,
            self.input
                .borrow()
                .parse_node(util::get_node("cutoff", nodes, 3)?)?,
        ))
    }
}
