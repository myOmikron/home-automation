use std::time::Duration;

use log::{error, trace};
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, ClientError, Event, MqttOptions};
use rumqttc::{TlsConfiguration, Transport};

use crate::config::Config;

/// Abstraction for the mqtt client.
///
/// Provides easy functions to send to the mqtt broker
#[derive(Clone)]
pub struct MqttClient(AsyncClient);

impl MqttClient {
    /// Publish a message to a topic
    pub async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), ClientError> {
        self.0
            .publish(topic, QoS::AtLeastOnce, true, payload.to_vec())
            .await
    }
}

/// Start the mqtt client
pub async fn start_mqtt_client(conf: &Config) -> Result<MqttClient, String> {
    let mut opts = MqttOptions::new("brain-publisher", conf.mqtt.host.clone(), conf.mqtt.port);
    opts.set_credentials(conf.mqtt.username.clone(), conf.mqtt.password.clone())
        .set_transport(Transport::Tls(TlsConfiguration::Native))
        .set_keep_alive(Duration::from_secs(5));

    let (client, mut event_loop) = AsyncClient::new(opts, 10);

    event_loop.poll().await.map_err(|e| e.to_string())?;

    let mqtt_client = MqttClient(client);

    tokio::spawn(async move {
        loop {
            match event_loop.poll().await {
                Ok(event) => match event {
                    Event::Incoming(inc) => {
                        trace!("Incoming mqtt: {inc:?}");
                    }
                    Event::Outgoing(out) => {
                        trace!("Outgoing mqtt: {out:?}");
                    }
                },
                Err(err) => {
                    error!("Connection error: {err}");
                    break;
                }
            }
        }
        error!("Mqtt event loop exited, exiting ..");
    });

    Ok(mqtt_client)
}
