use crate::adapters::{ContactRepository, Repository};
use crate::commands;
use crate::models::Contact;

pub struct ContactService<'a> {
    repository: &'a mut dyn ContactRepository,
}

impl<'a> ContactService<'a> {
    fn new(repo: &'a mut dyn ContactRepository) -> Self {
        ContactService { repository: repo }
    }

    async fn list(&self) -> Result<Vec<Contact>, String> {
        self.repository.list().await
    }

    async fn create_contact(&mut self, cmd: &commands::CreateContact) -> Result<Contact, String> {
        let contact = Contact {
            id: uuid::Uuid::new_v4().to_string(),
            name: cmd.name.clone(),
            email: cmd.email.clone(),
        };
        self.repository.create(&contact).await?;

        Ok(contact)
    }

    async fn update_contact(&mut self, cmd: &commands::UpdateContact) -> Result<Contact, String> {
        let mut contact = self.repository.get(&cmd.id).await.ok_or("Contact not found")?;
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
    use crate::adapters::{mock_repo, Model, mock_contact_repo};
    use crate::commands;
    use crate::models::Contact;
    use crate::services::contact_handlers::{Repository, ContactService};

    async fn _create_contact(service: &mut ContactService<'_>) -> Result<Contact, String> {
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
        let res = _create_contact(&mut service).await;
        let contacts = service.repository.list().await.unwrap();
        assert_eq!(contacts.len(), 1);
    }

    #[actix_web::test]
    async fn can_update_contact() {
        let mut repo = mock_contact_repo();
        let mut service = ContactService::new(&mut repo);
        let res = _create_contact(&mut service).await;
        let contacts = service.repository.list().await.unwrap();
        let id = contacts.first().unwrap().id.clone();

        let contact = service.repository.get(&id).await;
        assert!(contact.is_some());
        let contact = contact.unwrap();

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
        let res = _create_contact(&mut service).await;
        let contacts = service.repository.list().await.unwrap();
        let id = contacts.first().unwrap().id.clone();

        let res = service.repository.delete(&id).await;
        assert!(res.is_ok());
        let contacts = service.repository.list().await.unwrap();
        assert_eq!(contacts.len(), 0);
    }
}

#[cfg(test)]
mod tests_mongo {
    use crate::adapters::{MongoRepository};
    use crate::commands;
    use crate::services::ContactService;

    #[actix_web::test]
    async fn can_create_contact() {
        let db = crate::adapters::database::init("test").await;
        let mut repo = MongoRepository::new(&db, "contacts");
        let mut service = ContactService::new(&mut repo);
        let cmd = commands::CreateContact {
            name: "Jon Snow".to_string(),
            email: "jon@winterfell.com".to_string(),
        };
        let res = service.create_contact(&cmd).await;
        let contacts = service.repository.list().await.unwrap();
        assert_eq!(contacts.len(), 1);
    }
}
