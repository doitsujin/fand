use output::OutputCollection;
use sensor::SensorCollection;

use std::thread;
use std::time::Duration;

use app::Config;

pub struct App {
  sensors : SensorCollection,
  outputs : OutputCollection,
}

impl App {
  
  pub fn from_config(config: Config) -> App {
    App {
      sensors : config.sensors,
      outputs : config.outputs,
    }
  }
  
  pub fn run(&mut self) -> Result<(), String> {
    try!(self.outputs.enable_all(true));
    loop {
      try!(self.sensors.update_all());
      try!(self.outputs.update_all());
      thread::sleep(Duration::from_secs(1));
    }
  }
  
}