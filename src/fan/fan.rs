// Fan interface
pub trait Fan {
    // Specifies whether manual or automatic control
    // over this fan shall be enabled. The program
    // will only control 'enabled' outputs.
    fn set_enabled(&mut self, enabled: bool) -> Result<(), String>;

    // Sets the fan speed as a value between 0 and 1
    fn set(&mut self, v: f64) -> Result<(), String>;
}
