use crate::db::dbhandle::{dbmain, DatabaseType};
// use actix_web::web::Json;
use anyhow::{anyhow, Result}; // If using anyhow
use log::error;
use serde_json::{json, Value as JsonValue};

use crate::db::user_query::{
    check_db, day_mng_count, get_daily_stats, weeknd_mng_total, yesterday_count,
};

pub async fn check_connection() -> Result<JsonValue> {
    // Initialize database connections
    let main_connection = dbmain(DatabaseType::MainDb).await?;
    let warehouse_connection = dbmain(DatabaseType::WarehouseDb).await?;

    // Print connection details
    check_db(&main_connection, "Main Database").await?;
    check_db(&warehouse_connection, "Warehouse Database").await?;

    // Return success response with connection status
    Ok(json!({
        "status": "success",
        "connections": {
            "main_db": {
                "connected": true,
                "timestamp": chrono::Utc::now(),
                "pool_size": main_connection.size()
            },
            "warehouse_db": {
                "connected": true,
                "timestamp": chrono::Utc::now(),
                "pool_size": warehouse_connection.size()
            }
        }
    }))
}

pub async fn mng_count_day() -> Result<JsonValue> {
    let warehouse_connection = dbmain(DatabaseType::WarehouseDb).await?;
    let count = day_mng_count(&warehouse_connection).await?;

    Ok(json!({
        "status": "success",
        "connections": {
            "warehouse_db": {
                "count": count,
                "connected": true,
                "timestamp": chrono::Utc::now(),
                "pool_size": warehouse_connection.size()
            }
        }
    }))
}

pub async fn weekend_each_day() -> Result<JsonValue> {
    let warehouse_connection = dbmain(DatabaseType::WarehouseDb).await.map_err(|e| {
        error!("Failed to connect to warehouse DB: {}", e);
        anyhow!("Database connection failed: {}", e)
    })?;

    let stats = get_daily_stats(&warehouse_connection).await.map_err(|e| {
        error!("Failed to fetch daily stats: {}", e);
        anyhow!("Failed to fetch data: {}", e)
    })?;

    Ok(json!({
        "daily_stats": stats,
        "total_users": stats.iter().map(|s| s.user_count).sum::<i64>()
    }))
}

pub async fn weeknd_total_mng() -> Result<JsonValue> {
    let warehouse_connection = dbmain(DatabaseType::WarehouseDb).await?;
    let count = weeknd_mng_total(&warehouse_connection).await?;

    Ok(json!({
        "status": "success",
        "connections": {
            "warehouse_db": {
                "count": count,
                "connected": true,
                "timestamp": chrono::Utc::now(),
                "pool_size": warehouse_connection.size()
            }
        }
    }))
}

pub async fn yesterday_total() -> Result<JsonValue> {
    let warehouse_connection = dbmain(DatabaseType::WarehouseDb).await?;
    let stats = yesterday_count(&warehouse_connection).await.map_err(|e| {
        error!("Failed to fetch daily stats: {}", e);
        anyhow!("Failed to fetch data: {}", e)
    })?;
    Ok(json!({
        "created_orgs": stats.iter().map(|s| &s.created_org).collect::<Vec<_>>(),
        "total_users": stats.iter().map(|s| s.user_count).collect::<Vec<_>>()
    }))
}
