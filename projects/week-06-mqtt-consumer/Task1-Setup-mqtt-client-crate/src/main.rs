use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use tokio::time::{sleep, Duration};
use tracing::{error, info};

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
                    info!(
                        topic = %publish.topic,
                        payload = ?publish.payload,
                        "received publish message"
                    );
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