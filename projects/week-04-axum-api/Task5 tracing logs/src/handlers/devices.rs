use axum::{extract::State, Json};
use sqlx::SqlitePool;
use tracing::info;

use crate::models::device::Device;

pub async fn get_devices(
    State(pool): State<SqlitePool>,
) -> Json<Vec<Device>> {
    info!("GET /devices");

    let devices = sqlx::query_as::<_, Device>(
        r#"
        SELECT device_id, site_id, name, online
        FROM devices
        ORDER BY device_id
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("failed to fetch devices");

    Json(devices)
}