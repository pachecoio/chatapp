use crate::adapters::contact_repository::ContactRepository;
use crate::adapters::{IdType, Repository, RepositoryError};
use crate::commands;
use crate::models::Contact;

pub struct ContactService<'a> {
    repository: &'a mut dyn ContactRepository,
}

impl<'a> ContactService<'a> {
    pub fn new(repo: &'a mut dyn ContactRepository) -> Self {
        ContactService { repository: repo }
    }

    pub async fn list(
        &self,
        skip: Option<u64>,
        limit: Option<i32>,
    ) -> Result<(i32, Vec<Contact>), RepositoryError> {
        self.repository.list(skip, limit).await
    }

    pub async fn get(&self, id: &str) -> Option<Contact> {
        let id_type = IdType::String(id.to_string());
        self.repository.get(&id_type).await
    }

    pub async fn create_contact(
        &mut self,
        cmd: &commands::CreateContact,
    ) -> Result<Contact, RepositoryError> {
        let contact = Contact::new(&cmd.name, &cmd.email);
        match self.repository.find_by_email(&contact.email).await {
            Some(_) => {
                return Err(RepositoryError {
                    message: format!("Contact with email {} already exists", contact.email),
                })
            }
            None => (),
        }
        self.repository.create(&contact).await?;
        Ok(contact)
    }

    async fn update_contact(
        &mut self,
        cmd: &commands::UpdateContact,
    ) -> Result<Contact, RepositoryError> {
        let mut contact = match self.repository.get(&cmd.id).await {
            Some(c) => c,
            None => {
                return Err(RepositoryError {
                    message: "Contact not found".to_string(),
                })
            }
        };
        if let Some(name) = &cmd.name {
            contact.name = name.clone();
        }
        if let Some(email) = &cmd.email {
            contact.email = email.clone();
        }
        self.repository.update(&contact).await?;
        Ok(contact)
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::{mock_contact_repo, Model, RepositoryError};
    use crate::commands;
    use crate::models::Contact;
    use crate::services::contact_handlers::{ContactService, Repository};

    async fn _create_contact(service: &mut ContactService<'_>) -> Result<Contact, RepositoryError> {
        let cmd = commands::CreateContact {
            name: "Jon Snow".to_string(),
            email: "jon@winterfell.com".to_string(),
        };
        service.create_contact(&cmd).await
    }

    #[actix_web::test]
    async fn can_create_contact() {
        let mut repo = mock_contact_repo();
        let mut service = ContactService::new(&mut repo);
        let _res = _create_contact(&mut service).await;
        let (total, contacts) = service.repository.list(None, None).await.unwrap();
        assert_eq!(contacts.len(), 1);
    }

    #[actix_web::test]
    async fn can_update_contact() {
        let mut repo = mock_contact_repo();
        let mut service = ContactService::new(&mut repo);
        let _res = _create_contact(&mut service).await;
        let (total, contacts) = service.repository.list(None, None).await.unwrap();
        let id = contacts.first().unwrap().id();

        let contact = service.repository.get(&id).await;
        assert!(contact.is_some());
        let _contact = contact.unwrap();

        let cmd = commands::UpdateContact {
            id: id.clone(),
            name: Some("Arya Stark".to_string()),
            email: None,
        };
        let res = service.update_contact(&cmd).await;
        assert!(res.is_ok());

        let contact = service.repository.get(&id).await.unwrap();
        assert_eq!(contact.name, "Arya Stark");
    }

    #[actix_web::test]
    async fn can_delete_contact() {
        let mut repo = mock_contact_repo();
        let mut service = ContactService::new(&mut repo);
        let _res = _create_contact(&mut service).await;
        let (total, contacts) = service.repository.list(None, None).await.unwrap();
        let id = contacts.first().unwrap().id();

        let res = service.repository.delete(&id).await;
        assert!(res.is_ok());
        let (total, contacts) = service.repository.list(None, None).await.unwrap();
        assert_eq!(contacts.len(), 0);
    }
}

#[cfg(test)]
mod tests_mongo {
    use crate::adapters::mongo::repository::MongoRepository;
    use crate::adapters::Model;
    use crate::commands;
    use crate::services::ContactService;

    #[actix_web::test]
    #[ignore]
    async fn can_create_contact() {
        let db = crate::adapters::mongo::database::init("test").await;
        let mut repo = MongoRepository::new(&db, "contacts");
        let mut service = ContactService::new(&mut repo);
        let cmd = commands::CreateContact {
            name: "Samwell Tarly".to_string(),
            email: "samwell@thewall.com".to_string(),
        };
        let res = service.create_contact(&cmd).await;
        assert!(res.is_ok());

        // Cannot create duplicated contact
        let res_err = service.create_contact(&cmd).await;
        assert!(res_err.is_err());

        let (total, contacts) = service.repository.list(None, None).await.unwrap();
        assert!(!contacts.is_empty());

        let id = res.unwrap().id();
        let contact = service.repository.get(&id).await;
        assert!(contact.is_some());

        let mut contact = contact.unwrap();
        contact.name = "Arya Stark".to_string();

        let updated = service.repository.update(&contact).await;
        assert!(updated.is_ok());

        let contact = service.repository.get(&id).await.unwrap();
        assert_eq!(contact.name, "Arya Stark");

        let deleted = service.repository.delete(&id).await;
        assert!(deleted.is_ok());
    }
}
