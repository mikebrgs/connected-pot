// Local modules

// Local imports
use super::MessageBroker;

// Public imports
use paho_mqtt as mqtt;
use log;


pub struct MQTT {
    client: mqtt::client::Client
}


impl MQTT {
    pub fn new(broker: String, id: String) -> Result<MQTT, Box<dyn std::error::Error>> {
        // Create client options
        let cli_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(&broker)
            .client_id(id)
            .finalize();
        match mqtt::Client::new(cli_opts) {
            Ok(client) => {
                Ok(MQTT{ client })
            },
            Err(err) => {
                log::error!("{}", err);
                Err(Box::new(err))
            }
        }
    }
}


impl MessageBroker for MQTT {
    fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Connecting to MQTT broker.");
        match self.client.connect(None) {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("{}", err);
                Err(Box::new(err))
            }
        }
    }

    fn publish(&self, topic: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Publishing to topic {} the message {}", topic, message);
        let message = mqtt::Message::new(
            "pot/health",
            message.clone(),
            0
        );
        match self.client.publish(message) {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("{}", err);
                Err(Box::new(err))
            }
        }
    }

    fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Disconnecting from the MQTT broker");
        match self.client.disconnect(None) {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("{}", err);
                Err(Box::new(err))
            }
        }
    }
}