use crate::parser::{Evaluator, Node};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::sensor::Sensor;
use crate::util;

// Hwmon sensor
//
// An arbitrary (temperature) input of a hwmon device.
// While this can be used to monitor pretty much anything,
// it should only be used for temperature inputs as it will
// perform a conversion to actual degrees celsius.
pub struct HwmonSensor {
    path: PathBuf,
    value: Option<f64>,
}

impl HwmonSensor {
    pub fn create(hwmon: &str, input: &str) -> Rc<RefCell<Box<dyn Sensor>>> {
        let base_path = if util::HWMON_NAME_TO_PATH.contains_key(hwmon) {
            let hwmon_path = util::HWMON_NAME_TO_PATH.get(hwmon).expect("Hwmon label vanished between `contains_key` check and retrieval.");
            format!("{}/{}", hwmon_path, input)
        } else {
            warn!("Using old fallback for hwmon-input '{}'", hwmon);
            format!("/sys/class/hwmon/{}/{}", hwmon, input)
        };

        let mut path_v = PathBuf::new();
        path_v.push(Path::new(&base_path));

        Rc::new(RefCell::new(Box::new(HwmonSensor {
            path: path_v,
            value: None,
        })))
    }

    fn pass_char(&self, c: char) -> bool {
        match c {
            '0'..='9' | '-' => true,
            _ => false,
        }
    }
}

impl Sensor for HwmonSensor {
    fn value(&self) -> f64 {
        self.value.unwrap()
    }

    fn update(&mut self) -> Result<(), String> {
        let raw_str = util::read_text_file(&self.path)?;
        let val_str = raw_str
            .chars()
            .filter(|c| self.pass_char(*c))
            .collect::<String>();

        let raw_value = val_str
            .parse::<f64>()
            .map_err(|_| "Invalid number".to_string())?;
        self.value = Some(raw_value / 1000.0);
        Ok(())
    }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////

pub struct EvalHwmonSensor;

impl EvalHwmonSensor {
    pub fn new() -> EvalHwmonSensor {
        EvalHwmonSensor {}
    }
}

impl Evaluator<Rc<RefCell<Box<dyn Sensor>>>> for EvalHwmonSensor {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<Rc<RefCell<Box<dyn Sensor>>>, String> {
        Ok(HwmonSensor::create(
            util::get_text_node("hwmon-sensor", nodes, 0)?,
            util::get_text_node("hwmon-sensor", nodes, 1)?,
        ))
    }
}
