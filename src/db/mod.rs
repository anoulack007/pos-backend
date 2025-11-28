use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::log;

pub async fn connect_db(url: &str) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .sqlx_slow_statements_logging_settings(log::LevelFilter::Warn, Duration::from_secs(1));

    let db = Database::connect(opt).await?;
    Ok(db)
}
