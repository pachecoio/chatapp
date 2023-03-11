
use crate::{AppState};
use actix_web::{get, web, Error, HttpResponse};


#[get("/contacts")]
pub async fn get_contacts(_data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    // let db = &data.db;
    // let repo = MongoRepository {
    //     db: &db,
    // };
    // let service = ContactService::new(repo);
    // let contacts = service.list().await;
    // Ok(HttpResponse::Ok().json(contacts))
    todo!()
}
