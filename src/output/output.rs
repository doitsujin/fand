use fan::Fan;
use input::Input;

pub struct Output {
  fan   : Box<Fan>,
  input : Box<Input>,
}

impl Output {
  
  pub fn new(fan_v: Box<Fan>, input_v: Box<Input>) -> Output {
    Output { fan : fan_v, input : input_v }
  }
  
  pub fn set_enabled(&mut self, state: bool) -> Result<(), String> {
    self.fan.set_enabled(state)
  }
  
  // Computes and sets the fan's speed
  pub fn update(&mut self) -> Result<(), String> {
    self.fan.set(self.input.compute())
  }
  
}