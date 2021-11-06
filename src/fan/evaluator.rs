use std::collections::HashMap;
use std::rc::Rc;

use crate::fan::fan_console::EvalConsoleFan;
use crate::fan::fan_hwmon_pwm::EvalHwmonPwmFan;
use crate::fan::Fan;
use crate::parser::{Evaluator, Node, TagEvaluator};
use crate::util;

pub type NamedFans = HashMap<String, Box<dyn Fan>>;

// Fan evaluator
//
// Evaluates an entry for a named fan. Automatically
// creates the necessary tag evaluator during creation.
pub struct FanEvaluator {
    tag_evaluator: TagEvaluator<Box<dyn Fan>>,
}

impl FanEvaluator {
    pub fn new() -> FanEvaluator {
        let mut tag_evaluator_v: TagEvaluator<Box<dyn Fan>> = TagEvaluator::new();
        tag_evaluator_v.add("console-fan", Rc::new(EvalConsoleFan::new()));
        tag_evaluator_v.add("hwmon-pwm", Rc::new(EvalHwmonPwmFan::new()));

        FanEvaluator {
            tag_evaluator: tag_evaluator_v,
        }
    }
}

impl Evaluator<(String, Box<dyn Fan>)> for FanEvaluator {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<(String, Box<dyn Fan>), String> {
        let name = util::get_text_node("fan", nodes, 0)?;
        let node = util::get_node("fan", nodes, 1)?;
        Ok((name.clone(), self.tag_evaluator.parse_node(node)?))
    }
}
