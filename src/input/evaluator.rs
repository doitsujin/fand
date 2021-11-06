use std::cell::RefCell;
use std::rc::Rc;

use crate::input::Input;
use crate::parser::evaluator::TagEvaluator;
use crate::sensor::evaluator::NamedSensors;

use crate::input::cutoff::EvalCutoff;
use crate::input::maximum::EvalMaximum;
use crate::input::panic::EvalPanic;
use crate::input::sensor::EvalSensorInput;
use crate::input::smooth::EvalSmooth;
use crate::input::step::EvalSteps;

// Reference to input evaluator
pub type InputEvaluatorRef = Rc<RefCell<InputEvaluator>>;
pub type InputEvaluator = TagEvaluator<Box<dyn Input>>;

impl InputEvaluator {
    pub fn create(named_sensors: Rc<RefCell<NamedSensors>>) -> InputEvaluatorRef {
        let input_evaluator = Rc::new(RefCell::new(InputEvaluator::new()));
        InputEvaluator::init(input_evaluator.clone(), named_sensors);
        input_evaluator
    }

    fn init(
        input_evaluator: Rc<RefCell<InputEvaluator>>,
        named_sensors: Rc<RefCell<NamedSensors>>,
    ) {
        let mut input_borrow = input_evaluator.borrow_mut();
        input_borrow.add("cutoff", Rc::new(EvalCutoff::new(input_evaluator.clone())));
        input_borrow.add(
            "maximum",
            Rc::new(EvalMaximum::new(input_evaluator.clone())),
        );
        input_borrow.add("panic", Rc::new(EvalPanic::new(input_evaluator.clone())));
        input_borrow.add("sensor-input", Rc::new(EvalSensorInput::new(named_sensors)));
        input_borrow.add("smooth", Rc::new(EvalSmooth::new(input_evaluator.clone())));
        input_borrow.add("steps", Rc::new(EvalSteps::new(input_evaluator.clone())));
    }
}
