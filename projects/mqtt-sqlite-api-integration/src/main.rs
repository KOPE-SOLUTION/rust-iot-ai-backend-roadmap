use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;

mod db;

mod handlers {
    pub mod devices;
    pub mod health;
    pub mod telemetry;
}

mod models {
    pub mod device;
    pub mod telemetry;
}

mod mqtt;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = db::init_db().await;

    let mqtt_pool = pool.clone();
    tokio::spawn(async move {
        mqtt::consumer::run_mqtt_consumer(mqtt_pool).await;
    });

    let app = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/devices", get(handlers::devices::get_devices))
        .route("/telemetry/latest", get(handlers::telemetry::latest_telemetry))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    info!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}