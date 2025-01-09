use crate::db::settings::Settings;
use anyhow::Result;
use deadpool_postgres::{Client, Config, Pool, PoolError, Runtime};
use std::sync::Arc;
use tokio_postgres::NoTls;

pub struct Database {
    pub pool: Pool,
}

pub struct DbConnection {
    pub database: Arc<Database>,
}
impl Database {
    pub fn new(host: &str, port: u16, user: &str, password: &str, dbname: &str) -> Self {
        let mut cfg = Config::new();
        cfg.host = Some(host.to_string());
        cfg.port = Some(port);
        cfg.user = Some(user.to_string());
        cfg.password = Some(password.to_string());
        cfg.dbname = Some(dbname.to_string());

        let pool = cfg
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Failed to create pool");

        Database { pool }
    }

    pub async fn get_client(&self) -> Result<Client, PoolError> {
        self.pool.get().await
    }
}
pub fn connect_database(settings: Settings) -> (Option<DbConnection>, Option<DbConnection>) {
    let main_client = settings.main_db.map(|db| {
        let database = Database::new(&db.host, db.port, &db.user, &db.password, &db.name);

        DbConnection {
            database: Arc::new(database),
        }
    });

    let warehouse_client = settings.warehouse_db.map(|db| {
        let database = Database::new(&db.host, db.port, &db.user, &db.password, &db.name);

        DbConnection {
            database: Arc::new(database),
        }
    });

    (main_client, warehouse_client)
}

pub async fn start_database(database: Arc<Database>) {
    match database.get_client().await {
        Ok(_) => {
            println!("Client database connection successful");
        }
        Err(e) => {
            println!("Failed to connect to client database: {}", e);
            return;
        }
    }
}
