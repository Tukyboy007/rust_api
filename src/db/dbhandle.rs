use crate::models::models::Settings;
use anyhow::Result;
use log::{error, info};
use sqlx::{Pool, Postgres};

pub enum DatabaseType {
    MainDb,
    WarehouseDb,
}

// Return type should be Pool<Postgres> for PostgreSQL connections
pub async fn dbmain(db_type: DatabaseType) -> Result<Pool<Postgres>> {
    let settings = Settings::new().map_err(|e| {
        error!("Failed to load settings: {}", e);
        e
    })?;

    match db_type {
        DatabaseType::MainDb => {
            if let Some(db) = &settings.main_db {
                info!("📍 DbHost: {}", db.host);
                println!("📍 Port: {}", db.port);
                println!("📍 User: {}", db.user);
                println!("📍 DB Name: {}", db.name);

                // Build connection string
                let database_url = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    db.user, db.password, db.host, db.port, db.name
                );

                // Create the pool using SQLx
                let pool = sqlx::postgres::PgPool::connect(&database_url).await?;
                Ok(pool)
            } else {
                anyhow::bail!("Main database configuration not found")
            }
        }
        DatabaseType::WarehouseDb => {
            if let Some(db) = &settings.warehouse_db {
                info!("📍 DbHost: {}", db.host);
                println!("📍 Port: {}", db.port);
                println!("📍 User: {}", db.user);
                println!("📍 DB Name: {}", db.name);

                let database_url = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    db.user, db.password, db.host, db.port, db.name
                );

                let pool = sqlx::postgres::PgPool::connect(&database_url).await?;
                Ok(pool)
            } else {
                anyhow::bail!("Warehouse database configuration not found")
            }
        }
    }
}
