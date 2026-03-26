use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use serde::Deserialize;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

#[derive(Debug, Deserialize)]
struct TelemetryPayload {
    device_id: String,
    sensor: String,
    value: f64,
    unit: String,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut mqttoptions = MqttOptions::new("rust-consumer-01", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe("site/+/device/+/telemetry/+", QoS::AtMostOnce)
        .await
        .expect("failed to subscribe");

    info!("MQTT consumer started. Waiting for messages...");

    loop {
        match eventloop.poll().await {
            Ok(notification) => match notification {
                Event::Incoming(Incoming::Publish(publish)) => {
                    let payload_str = match std::str::from_utf8(&publish.payload) {
                        Ok(s) => s,
                        Err(err) => {
                            error!(error = %err, topic = %publish.topic, "invalid UTF-8 payload");
                            continue;
                        }
                    };

                    match serde_json::from_str::<TelemetryPayload>(payload_str) {
                        Ok(data) => {
                            info!(
                                topic = %publish.topic,
                                device_id = %data.device_id,
                                sensor = %data.sensor,
                                value = data.value,
                                unit = %data.unit,
                                timestamp = %data.timestamp,
                                "parsed telemetry payload"
                            );
                        }
                        Err(err) => {
                            error!(
                                error = %err,
                                topic = %publish.topic,
                                payload = %payload_str,
                                "failed to parse telemetry JSON"
                            );
                        }
                    }
                }
                other => {
                    info!(event = ?other, "mqtt event");
                }
            },
            Err(err) => {
                error!(error = %err, "mqtt connection error");
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}