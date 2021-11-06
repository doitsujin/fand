pub mod evaluator;
pub mod sensor;
pub mod sensor_collection;
pub mod sensor_hwmon;

pub use crate::sensor::sensor::Sensor;
pub use crate::sensor::sensor_collection::SensorCollection;
pub use crate::sensor::sensor_hwmon::HwmonSensor;
