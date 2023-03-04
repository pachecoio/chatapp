mod adapters;
pub mod commands;
mod models;
mod services;
mod websocket;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(websocket::index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
