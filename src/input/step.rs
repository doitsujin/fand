use crate::input::Input;

use crate::input::evaluator::InputEvaluatorRef;
use crate::parser::{Evaluator, Node};
use crate::util;

// Step in a step sequence
pub struct Step {
    pub v_in: f64,
    pub v_out: f64,
}

// Step sequence
pub struct Steps {
    input: Box<dyn Input>,
    steps: Vec<Step>,
}

impl Step {
    pub fn new(in_v: f64, out_v: f64) -> Step {
        Step {
            v_in: in_v,
            v_out: out_v,
        }
    }

    pub fn interpolate(a: &Step, b: &Step, x: f64) -> f64 {
        let mut v = x;

        let delta_in = b.v_in - a.v_in;
        let delta_out = b.v_out - a.v_out;

        v -= a.v_in;
        v /= delta_in;
        v *= delta_out;
        v += a.v_out;
        v
    }
}

impl Steps {
    pub fn create(steps_v: Vec<Step>, input_v: Box<dyn Input>) -> Box<dyn Input> {
        Box::new(Steps {
            input: input_v,
            steps: steps_v,
        })
    }
}

impl Input for Steps {
    fn compute(&mut self) -> f64 {
        let input = self.input.compute();

        // Index of first and last step
        let max_index: usize = self.steps.len() - 1;
        let mut index: usize = 0;

        // Borrow border steps for convenience
        let min_step = &self.steps[0];
        let max_step = &self.steps[max_index];

        // Exit early if the input is out of bounds
        if input <= min_step.v_in {
            return min_step.v_out;
        }
        if input >= max_step.v_in {
            return max_step.v_out;
        }

        // Unfortulately, this option is not available
        // now so we have to scan all the steps.
        while index < max_index - 1 && input >= self.steps[index + 1].v_in {
            index += 1;
        }

        Step::interpolate(&self.steps[index + 0], &self.steps[index + 1], input)
    }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////

pub struct EvalSteps {
    input: InputEvaluatorRef,
}

impl EvalSteps {
    pub fn new(input_v: InputEvaluatorRef) -> EvalSteps {
        EvalSteps { input: input_v }
    }
}

impl Evaluator<Box<dyn Input>> for EvalSteps {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<Box<dyn Input>, String> {
        let mut input: Option<Box<dyn Input>> = None;
        let mut steps: Vec<Step> = Vec::new();

        for n in nodes {
            match *n {
                Node::Node(ref s, ref nx) => {
                    if s == "step" {
                        steps.push(Step::new(
                            util::get_num_node::<f64>("step", nx, 0)?,
                            util::get_num_node::<f64>("step", nx, 1)?,
                        ));
                    } else if input.is_none() {
                        input = Some(self.input.borrow().parse_node(n)?);
                    } else {
                        return Err(format!("(step): Unexpected node '{}'", s));
                    }
                }
                Node::Text(_) => return Err("(step): Unexpected text node".to_string()),
            }
        }

        // Create the actual step object
        let input_node = input.ok_or("(step): Missing input".to_string())?;
        Ok(Steps::create(steps, input_node))
    }
}
