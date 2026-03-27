use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use sqlx::SqlitePool;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use crate::models::telemetry::Telemetry;

pub async fn run_mqtt_consumer(pool: SqlitePool) {
    let mut mqttoptions = MqttOptions::new("rust-integrated-consumer", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe("site/+/device/+/telemetry/+", QoS::AtMostOnce)
        .await
        .expect("failed to subscribe");

    info!("MQTT consumer started");

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

                    match serde_json::from_str::<Telemetry>(payload_str) {
                        Ok(data) => {
                            info!(
                                topic = %publish.topic,
                                device_id = %data.device_id,
                                sensor = %data.sensor,
                                value = data.value,
                                "parsed telemetry payload"
                            );

                            if let Err(err) = insert_telemetry(&pool, &data).await {
                                error!(error = %err, "failed to insert telemetry");
                            }
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

async fn insert_telemetry(pool: &SqlitePool, data: &Telemetry) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO telemetry_latest (device_id, sensor, value, unit, timestamp)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(&data.device_id)
    .bind(&data.sensor)
    .bind(data.value)
    .bind(&data.unit)
    .bind(&data.timestamp)
    .execute(pool)
    .await?;

    Ok(())
}