// Sensor trait
//
// Provides methods to read and retrieve the sensor's
// values. Each value can be read multiple times without
// having to actually re-read anything.
pub trait Sensor {
    // Retrieves sensor value
    fn value(&self) -> f64;

    // Updates sensor value
    fn update(&mut self) -> Result<(), String>;
}
