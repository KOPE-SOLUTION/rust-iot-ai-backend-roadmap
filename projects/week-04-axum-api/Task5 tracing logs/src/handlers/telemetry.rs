use axum::{extract::State, Json};
use sqlx::SqlitePool;
use tracing::info;

use crate::models::telemetry::Telemetry;

pub async fn latest_telemetry(
    State(pool): State<SqlitePool>,
) -> Json<Vec<Telemetry>> {
    info!("GET /telemetry/latest");

    let telemetry = sqlx::query_as::<_, Telemetry>(
        r#"
        SELECT device_id, sensor, value, unit, timestamp
        FROM telemetry_latest
        ORDER BY timestamp DESC
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("failed to fetch telemetry");

    Json(telemetry)
}