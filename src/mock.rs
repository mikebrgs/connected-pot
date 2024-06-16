// Local modules
mod models;
mod config;
mod message_broker;

// Local imports
use models::pot::PotHealth;
use message_broker::MessageBroker;
use config::Config;

// Public imports
use rand::prelude::*;
use scopeguard::defer;
use std::{fs::File, io::BufReader, thread, time};
use chrono::{DateTime, Utc};


fn main() {
    // Set logger
    env_logger::init();

    // Start random number generation
    let mut rng = thread_rng();

    // Load config
    let file = File::open("configs/pot-client.yaml")
        .expect("Failed to open config file");
    let reader = BufReader::new(file);

    // Deserialize the configuration from the file
    let config: Config = serde_yaml::from_reader(reader)
        .expect("Failed to parse config file");

    // Create and connect to message broker - MQTT
    let mut mqtt_client = message_broker::mqtt::MQTT::new(
        format!("mqtt://{}:{}", config.mqtt.host, config.mqtt.port),
        format!("{}-{}", "pot-client-mock", rng.gen_range(0..1000))
    ).unwrap();
    mqtt_client.connect().unwrap();
    defer! {
        mqtt_client.disconnect().unwrap();
    }

    loop {
        // Timestamp creation
        let timestamp = time::SystemTime::now();
        let timestamp: DateTime<Utc> = timestamp.into();

        // Generate sensor values
        let temperature_celsius: f32 = rng.gen::<f32>() * 30.0;
        let humidity: f32 = rng.gen();
        let pressure_bar: f32 = rng.gen();
        let moisture_level: f32 = rng.gen::<f32>() * 10_000.0;
        let light_level: f32 = rng.gen::<f32>() * 20_000.0;
        
        let pot_health = PotHealth{
            ts: timestamp,
            device_id: config.device_id.clone(),
            temperature: temperature_celsius,
            humidity: humidity,
            pressure: pressure_bar,
            moisture: moisture_level,
            light: light_level
        };

        let serialized = serde_json::to_string(&pot_health).unwrap();
        mqtt_client.publish("pot/health", &serialized).unwrap();

        thread::sleep(time::Duration::from_secs_f32(config.sensors.period))
    }

}