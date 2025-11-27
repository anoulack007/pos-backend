mod config;
mod db;
mod app;
mod entities;

use crate::config::Config;
use crate::db::connect_db;
use crate::app::create_app;

use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // โหลด .env
    dotenv().ok();

    // logger แบบง่าย ๆ
    tracing_subscriber::fmt().init();

    // โหลด config
    let config = Config::from_env()?;

    // connect DB
    let db = connect_db(&config.database_url).await?;

    // สร้าง app (Router)
    let app = create_app(db, config);

    // start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Server running at http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
