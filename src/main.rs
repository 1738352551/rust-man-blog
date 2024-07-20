use actix_web::{App, HttpServer, web};
use actix_web::dev::HttpServiceFactory;
use crate::config::routes;

mod model;
mod services;
mod api;
mod config;
mod error;
mod custom_resp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(||{
       App::new()
           .configure(routes::config_services)
   })
       .bind(("127.0.0.1", 8989))?
       .run()
       .await
}
