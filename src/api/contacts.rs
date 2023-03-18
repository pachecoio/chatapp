use crate::adapters::mongo::repository::MongoRepository;
use crate::commands::CreateContact;
use crate::services::ContactService;
use crate::AppState;
use actix_web::{get, post, web, Error, HttpResponse};
use serde_json::json;
use crate::models::Contact;
use serde::Deserialize;

pub fn get_scope() -> actix_web::Scope {
    web::scope("/contacts")
        .service(get_contacts)
        .service(create_contact)
}

#[derive(Deserialize)]
pub struct GetContactsQuery {
    page: Option<i32>,
    per_page: Option<i32>,
}

#[get("")]
pub async fn get_contacts(data: web::Data<AppState>, query: web::Query<GetContactsQuery>) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let db = &data.db;
    let mut repo = get_repository(db);
    let service = ContactService::new(&mut repo);
    let (total, contacts) = service.list(
        Some(((page - 1) * per_page) as u64),
        Some(per_page),
    ).await.unwrap();
    let response_data = json!({
        "page": page,
        "per_page": per_page,
        "total": total,
        "items": contacts,
    });
    Ok(HttpResponse::Ok().json(response_data))
}

#[get("/{id}")]
pub async fn get_contact(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    todo!()
}

#[post("")]
pub async fn create_contact(
    data: web::Data<AppState>,
    contact: web::Json<CreateContact>,
) -> Result<HttpResponse, Error> {
    let db = &data.db;
    let mut repo = get_repository(db);
    let mut service = ContactService::new(&mut repo);
    let res = service.create_contact(&contact).await;
    match res {
        Ok(contact) => Ok(HttpResponse::Ok().json(contact)),
        Err(e) => Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .json(e)),
    }
}

fn get_repository(db: &mongodb::Database) -> MongoRepository<Contact> {
    MongoRepository::new(db, "contacts")
}

#[cfg(test)]
mod integration_tests {
    use actix_web::{App, test, web};
    use crate::{adapters, AppState};
    use crate::api::contacts::get_scope;

    #[actix_web::test]
    async fn test_get_contacts() -> Result<(), actix_web::Error> {
        let db = adapters::mongo::database::init("chatapp").await;
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { db: db.to_owned() }))
                .service(get_scope()),
        )
        .await;
        let req = test::TestRequest::get().uri("/contacts").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        Ok(())
    }
}