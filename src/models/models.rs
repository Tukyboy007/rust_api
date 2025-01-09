use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

pub struct Settings {
    pub host: String,
    pub port: u16,
    pub main_db: Option<MainDb>,           // Optional main database
    pub warehouse_db: Option<WarehouseDb>, // Optional warehouse database
}

#[derive(Debug, Deserialize)]
pub struct MainDb {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct WarehouseDb {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct DailyStats {
    pub day: NaiveDate,
    pub user_count: i64,
}

pub struct CreatedOrg {
    pub created_org: String,
    pub user_count: i64,
}
