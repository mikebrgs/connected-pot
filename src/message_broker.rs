// Local modules
pub mod mqtt;

// Local imports

// Public imports


/// Trait for a message broker to publish messages to MQTT, or other formats.
pub trait MessageBroker {
    /// Connect to message broker.
    fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Publish a message.
    fn publish(&self, topic: &str, message: &str) -> Result<(), Box<dyn std::error::Error>>;

    // Disconnet from the broker.
    fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>>;
}