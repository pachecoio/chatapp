use actix_web::{Error, HttpResponse, web, get};
use serde_json::json;
use crate::{AppState, services};
use crate::services::ContactService;

#[get("/contacts")]
pub async fn get_contacts(
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // let db = &data.db;
    // let repo = MongoRepository {
    //     db: &db,
    // };
    // let service = ContactService::new(repo);
    // let contacts = service.list().await;
    // Ok(HttpResponse::Ok().json(contacts))
    todo!()
}