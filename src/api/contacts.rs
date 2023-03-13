use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use crate::adapters::mongo::repository::MongoRepository;
use crate::commands::CreateContact;
use crate::services::ContactService;

#[get("/contacts")]
pub async fn get_contacts(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let db = &data.db;
    let mut repo = MongoRepository::new(db, "contacts");
    let service = ContactService::new(&mut repo);
    let contacts = service.list().await.unwrap();
    Ok(HttpResponse::Ok().json(contacts))
}

#[post("/contacts")]
pub async fn create_contact(data: web::Data<AppState>, contact: web::Json<CreateContact>) -> Result<HttpResponse, Error> {
    let db = &data.db;
    let mut repo = MongoRepository::new(db, "contacts");
    let mut service = ContactService::new(&mut repo);
    let res = service.create_contact(&contact).await;
    match res {
        Ok(contact) => Ok(HttpResponse::Ok().json(contact)),
        Err(e) => Ok(
            HttpResponse::BadRequest()
                .content_type("application/json")
                .json(e)
        ),
    }
}
