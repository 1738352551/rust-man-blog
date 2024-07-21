use crate::config::routes;
use actix_web::{web, App, HttpServer};
use sea_orm::{prelude::ChronoDateTimeLocal, ConnectOptions, Database, DatabaseConnection};
use tracing::{debug, info};
use tracing_subscriber::fmt::time::ChronoLocal;
use std::sync::{Arc};
use tokio::sync::Mutex;
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
    // 初始化日志记录
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_owned()))
        .init();

    // 读取配置文件
    let config = config::yml::read_config("config.yml").expect("读取配置文件失败!");

    let mut opt = ConnectOptions::new(&config.database.url);

    opt.sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Info);

    // 建立数据库连接
    let db = Database::connect(opt)
        .await
        .expect("数据库连接失败!");

    print_startup();
    info!("初始化配置完成!");

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

fn print_startup() {
    info!("                                                                                                                                                      
                                                                |                                                                                                  
                       ||                   ||                  || |||            |||                  |||                      ||                                 
              ||| ||||||||         ||| ||||||||         |||     |||||||            ||                   |||                     |||                   |            
               || ||| | |||         || ||| | |||         ||   |||||| |        |    || ||||               || ||                  ||                  |||       |||  
               ||  |||||||          ||  |||||||          ||    ||| ||         |||||||||||||         || ||||||||           |||   ||                   ||       |||  
               ||| || |||           ||| || |||           || | ||||||||       |||||||   ||||         ||||||  |||        ||||||   || ||                ||       |||  
               |||  |||||           |||  |||||           |||||||||| ||       ||  ||||||||           |||     ||               |||||||||               ||        ||  
            || |||||||||||||     || |||||||||||||    |||||||||||||| |        || |||||||             |||    |||            |||||||  |||      |||      ||        ||  
            || ||||||||||||      || ||||||||||||      || ||  ||||||||          |||  ||              ||||||||||       ||||||||| ||  ||     ||| |||    || |||||  ||  
            || || | ||||||       || || | ||||||          ||  ||||| ||         ||  |||               ||||||||         ||||||    ||  ||    |||   |||   ||  ||    ||  
            |  || ||   ||        |  || ||   ||           ||  || || |||||         ||||||            |||     |||           | ||  ||  ||    ||     ||   || ||     ||  
               ||  ||||||           ||  ||||||           ||   |||||||||||       |||  ||||          ||||||||||||         ||||||||   ||    ||     ||   ||||      ||  
               ||   |  ||           ||   |  ||           ||||||||  ||          |||  || |||||||    |||||     |||        |||||||||   ||    ||     ||   |||||     ||  
               ||    |||            ||    |||            ||    ||  ||        ||||||||||||||||     || ||     ||        ||||  |||    ||    ||     ||   || |||        
               ||    ||||           ||    ||||          |||    ||  ||      |||  ||   |||         ||  ||   |||          |    ||| |||||     ||   ||   ||| ||||  |||  
              ||| |||| |||||       ||| |||| |||||       |||        ||           ||   ||         |||  |||||||||              ||  ||||       |||||    |||||||||||||  
              |||||||   ||||||     |||||||   ||||||     ||       ||||           |||||||        ||    ||                   |||    ||                                
               |                    |                    |        ||            ||||||         |                          |                                        
                                                                   |            ||                                                                                 
    ");
}