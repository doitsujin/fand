use crate::fan::Fan;
use crate::input::Input;

pub struct Output {
    fan: Box<dyn Fan>,
    input: Box<dyn Input>,
}

impl Output {
    pub fn new(fan_v: Box<dyn Fan>, input_v: Box<dyn Input>) -> Output {
        Output {
            fan: fan_v,
            input: input_v,
        }
    }

    pub fn set_enabled(&mut self, state: bool) -> Result<(), String> {
        self.fan.set_enabled(state)
    }

    // Computes and sets the fan's speed
    pub fn update(&mut self) -> Result<(), String> {
        self.fan.set(self.input.compute())
    }
}
