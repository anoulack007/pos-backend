use sea_orm::{Database, DatabaseConnection};

pub async fn connect_db(url: &str) -> anyhow::Result<DatabaseConnection> {
    let db = Database::connect(url).await?;
    Ok(db)
}

