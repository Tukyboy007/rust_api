use crate::models::models::{MainDb, Settings, WarehouseDb};

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        Ok(Settings {
            host: String::from("0.0.0.0"),
            port: 8080,
            main_db: Some(MainDb {
                host: String::from("localhost"),
                port: 5432,
                user: String::from("postgres"),
                password: String::from("123"),
                name: String::from("main_db"),
            }),
            warehouse_db: Some(WarehouseDb {
                host: String::from("localhost"),
                port: 5432,
                user: String::from("postgres"),
                password: String::from("123"),
                name: String::from("warehouse_db"),
            }),
        })
    }
}
