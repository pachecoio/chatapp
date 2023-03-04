mod websocket;
mod models;
mod services;
pub mod commands;
mod adapters;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(websocket::index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
