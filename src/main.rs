mod config;
mod db;
mod controller;
mod entities;
mod service;
mod middleware;

use config::logging::init_logging;
use crate::config::Config;
use crate::db::connect_db;
use crate::controller::create_app;

use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();
    dotenv().ok();
    let config = Config::from_env()?;
    let db = connect_db(&config.database_url).await?;

    let app = create_app(db, config);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Server running at http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
