use crate::parser::{Evaluator, Node};
use std::cmp;
use std::path::{Path, PathBuf};

use crate::fan::Fan;
use crate::util;

// Hwmon PWM fan
//
// Controls a PWM output of a hwmon device. Internally,
// these are represented as a text file, with possible
// values ranging from 0 to 255.
pub struct HwmonPwmFan {
    path_to_pwm: PathBuf,
    path_to_enable: PathBuf,
}

impl HwmonPwmFan {
    pub fn create(hwmon: &str, output: &str) -> Box<Fan> {
        let base_path_pwm = format!("/sys/class/hwmon/{}/{}", hwmon, output);
        let base_path_enable = format!("{}_enable", &base_path_pwm);

        let mut path_to_pwm_v = PathBuf::new();
        path_to_pwm_v.push(Path::new(&base_path_pwm));
        let mut path_to_enable_v = PathBuf::new();
        path_to_enable_v.push(Path::new(&base_path_enable));

        Box::new(HwmonPwmFan {
            path_to_pwm: path_to_pwm_v,
            path_to_enable: path_to_enable_v,
        })
    }
}

impl Fan for HwmonPwmFan {
    fn set_enabled(&mut self, enabled: bool) -> Result<(), String> {
        util::write_text_file(
            &self.path_to_enable,
            match enabled {
                true => "1",
                false => "0",
            },
        )
    }

    fn set(&mut self, v: f64) -> Result<(), String> {
        let v_i = cmp::max(cmp::min(255, (v * 255.0) as i32), 0);
        util::write_text_file(&self.path_to_pwm, &v_i.to_string())
    }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////

pub struct EvalHwmonPwmFan;

impl EvalHwmonPwmFan {
    pub fn new() -> EvalHwmonPwmFan {
        EvalHwmonPwmFan {}
    }
}

impl Evaluator<Box<Fan>> for EvalHwmonPwmFan {
    fn parse_nodes(&self, nodes: &[Node]) -> Result<Box<Fan>, String> {
        Ok(HwmonPwmFan::create(
            util::get_text_node("hwmon-pwm", nodes, 0)?,
            util::get_text_node("hwmon-pwm", nodes, 1)?,
        ))
    }
}
