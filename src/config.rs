// Local modules

// Local imports

// Public imports
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub sensors: Sensors,
    pub mqtt: MqttConfig,
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MqttConfig {
    pub host: String,
    pub port: i16
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensors {
    pub period: f32,
    pub atmospheric: AtmosphericSensor,
    pub moisture: MoistureSensor,
    pub light: LightSensor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AtmosphericSensor {
    pub address: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoistureSensor {
    pub address: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightSensor {
    pub address: String
}