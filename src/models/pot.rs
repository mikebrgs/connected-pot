// Local modules

// Local imports

// Public imports
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Debug)]
pub struct PotHealth {
    pub ts: DateTime<Utc>,
    pub device_id: String,
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub moisture: f32,
    pub light: f32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PotStatus {
    pub ts: DateTime<Utc>,
    pub device_id: String,
    pub battery: f32,
    pub memory: f32,
    pub cpu: f32,
    pub temperature: f32,
    pub storage: f32
}
