use axum::{
    routing::get,
    Router,
};

use tokio::net::TcpListener;

mod handlers {
    pub mod health;
    pub mod devices;
    pub mod telemetry;
}

mod models {
    pub mod device;
    pub mod telemetry;
}

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/devices", get(handlers::devices::get_devices))
        .route("/telemetry/latest", get(handlers::telemetry::latest_telemetry));

    let listener =
        TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}