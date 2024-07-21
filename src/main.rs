use crate::config::routes;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use std::sync::{Arc, Mutex};

mod api;
mod config;
mod custom_resp;
mod error;
mod model;
mod services;

struct AppState {
    db: Arc<Mutex<DatabaseConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 读取配置文件
    let config = config::yml::read_config("config.yml").expect("读取配置文件失败!");

    // 建立数据库连接
    let db = Database::connect(&config.database.url)
        .await
        .expect("数据库连接失败!");

    let app_state = web::Data::new(AppState {
        db: Arc::new(Mutex::new(db)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::config_services)
    })
    .bind((config.server.address.as_str(), config.server.port))?
    .run()
    .await
}
