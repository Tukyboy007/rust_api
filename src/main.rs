mod db;
mod event;
mod models;
mod routes;

use actix_web::{App, HttpServer};
use log::LevelFilter;

use crate::routes::user_routes::routes::{configure_health, configure_user};
use models::models::Settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let settings = Settings::new().expect("Failed to load settings");

    HttpServer::new(|| {
        App::new()
            .configure(configure_health)
            .configure(configure_user)
    })
    .bind((settings.host, settings.port))?
    .run()
    .await
}
