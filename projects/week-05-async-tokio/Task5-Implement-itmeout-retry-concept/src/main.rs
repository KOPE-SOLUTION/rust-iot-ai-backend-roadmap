use tokio::time::{sleep, timeout, Duration};
use tracing::{info, warn};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Starting timeout/retry demo...");

    let result = read_with_retry("node-01", 2, 2).await;
    info!("Result for node-01 => {:?}", result);

    let result = read_with_retry("node-02", 5, 2).await;
    info!("Result for node-02 => {:?}", result);
}

async fn read_with_retry(
    device_id: &str,
    response_delay_secs: u64,
    max_retries: u32,
) -> Result<f64, String> {
    let mut attempt = 0;

    while attempt <= max_retries {
        attempt += 1;

        info!("{} -> attempt {}", device_id, attempt);

        let result = timeout(
            Duration::from_secs(3),
            read_sensor(device_id, response_delay_secs),
        )
        .await;

        match result {
            Ok(value) => {
                info!("{} -> success on attempt {}", device_id, attempt);
                return Ok(value);
            }
            Err(_) => {
                warn!("{} -> timeout on attempt {}", device_id, attempt);

                if attempt > max_retries {
                    return Err(format!(
                        "{} failed after {} attempts",
                        device_id, attempt
                    ));
                }
            }
        }
    }

    Err("unexpected retry failure".to_string())
}

async fn read_sensor(device_id: &str, delay_secs: u64) -> f64 {
    info!("{} -> reading sensor...", device_id);
    sleep(Duration::from_secs(delay_secs)).await;
    42.0
}