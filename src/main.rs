mod adapters;
mod api;
pub mod commands;
mod models;
mod services;
mod websocket;

use actix_web::{web, App, HttpServer};

pub struct AppState {
    db: mongodb::Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = adapters::mongo::database::init("chatapp").await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.to_owned() }))
            .service(api::contacts::get_scope())
            .route("/ws/", web::get().to(websocket::index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
