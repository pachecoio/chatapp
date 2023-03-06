mod adapters;
pub mod commands;
mod models;
mod services;
mod websocket;
mod api;

use actix_web::{web, App, HttpServer};

struct AppState {
    db: mongodb::Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = adapters::database::init("chatapp").await;
    HttpServer::new(move || App::new()
        .app_data(web::Data::new(AppState {
            db: db.to_owned(),
        }))
        .service(api::contacts::get_contacts)
        .route("/ws/", web::get().to(websocket::index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
