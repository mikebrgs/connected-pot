// Local modules
mod models;
mod config;
mod message_broker;

// Local imports
use models::pot::PotHealth;
use message_broker::MessageBroker;
use config::Config;

// Local-ish imports
use atmospheric_sensor;
use light_sensor;
use moisture_sensor;

// Public imports
use rand::prelude::*;
use linux_embedded_hal::{Delay, I2cdev};
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

    // Load sensors
    let mut atmospheric_sensor = atmospheric_sensor::AtmosphericSensor::build(
        I2cdev::new("/dev/i2c-1").expect("Failed to connect to atmospheric sensor"),
        atmospheric_sensor::Address::Alternative
    );
    let mut moisture_sensor = moisture_sensor::MoistureSensor::build(
        I2cdev::new("/dev/i2c-1").expect("Failed to connect to moisture sensor"),
        moisture_sensor::Address::Default
    );
    let mut light_sensor = light_sensor::LightSensor::build(
        I2cdev::new("/dev/i2c-1").expect("Failed to connect to light sensor"),
        light_sensor::Address::Default.into(),
        Delay{}
    );

    // Wait for 1 second to get the connections setup
    thread::sleep(time::Duration::from_secs(1));

    loop {
        // Timestamp creation
        let timestamp = time::SystemTime::now();
        let timestamp: DateTime<Utc> = timestamp.into();

        // Generate sensor values
        let temperature_celsius = atmospheric_sensor.get_temperature_celsius().unwrap() as f32;
        let humidity = atmospheric_sensor.get_humidity_relative().unwrap() as f32;
        let pressure_bar = atmospheric_sensor.get_pressure_pascal().unwrap() as f32;
        let moisture_level = moisture_sensor.get_moisture_level().unwrap() as f32;
        let light_level = light_sensor.get_ambient_light_lux().unwrap() as f32;

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
