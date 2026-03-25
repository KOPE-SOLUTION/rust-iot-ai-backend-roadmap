use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::{path::PathBuf, str::FromStr};

pub async fn init_db() -> SqlitePool {
    let db_path = PathBuf::from("iot.db");

    let options = SqliteConnectOptions::from_str(
        db_path.to_str().expect("invalid db path"),
    )
    .expect("failed to build sqlite options")
    .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("failed to connect to SQLite");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS devices (
            device_id TEXT PRIMARY KEY,
            site_id TEXT NOT NULL,
            name TEXT NOT NULL,
            online BOOLEAN NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("failed to create devices table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS telemetry_latest (
            device_id TEXT NOT NULL,
            sensor TEXT NOT NULL,
            value REAL NOT NULL,
            unit TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("failed to create telemetry_latest table");

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO devices (device_id, site_id, name, online)
        VALUES
            ('node-01', 'farm-1', 'Greenhouse Temp Node', 1),
            ('node-02', 'farm-1', 'Humidity Node', 0)
        "#,
    )
    .execute(&pool)
    .await
    .expect("failed to seed devices");

    sqlx::query(
        r#"
        INSERT INTO telemetry_latest (device_id, sensor, value, unit, timestamp)
        VALUES
            ('node-01', 'temp', 27.5, 'C', '2026-03-26T10:00:00Z'),
            ('node-02', 'humidity', 68.2, '%', '2026-03-26T10:01:00Z')
        "#,
    )
    .execute(&pool)
    .await
    .ok();

    pool
}