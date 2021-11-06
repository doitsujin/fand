use crate::output::OutputCollection;
use crate::sensor::SensorCollection;

use std::thread;
use std::time::Duration;

use crate::app::Config;

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
    r#try!(self.outputs.enable_all(true));
    loop {
      r#try!(self.sensors.update_all());
      r#try!(self.outputs.update_all());
      thread::sleep(Duration::from_secs(1));
    }
  }
  
}