use crate::adapters::Repository;
use crate::commands;
use crate::models::Contact;


pub struct ContactService {
    repository: Box<dyn Repository<Contact>>,
}

impl ContactService {
    pub fn new<R: Repository<Contact> + 'static>(repository: R) -> Self {
        ContactService {
            repository: Box::new(repository),
        }
    }

    pub async fn list(&self) -> Vec<Contact> {
        self.repository.list().unwrap()
    }

    pub async fn create_contact(
        &mut self,
        cmd: &commands::CreateContact,
    ) -> Result<Contact, String> {
        let contact = Contact {
            id: uuid::Uuid::new_v4().to_string(),
            name: cmd.name.clone(),
            email: cmd.email.clone(),
        };
        self.repository.create(&contact)?;

        Ok(contact)
    }

    pub async fn update_contact(
        &mut self,
        cmd: &commands::UpdateContact,
    ) -> Result<Contact, String> {
        let mut contact = self.repository.get(&cmd.id).ok_or("Contact not found")?;
        if let Some(name) = &cmd.name {
            contact.name = name.clone();
        }
        if let Some(email) = &cmd.email {
            contact.email = email.clone();
        }
        self.repository.update(&contact)?;
        Ok(contact)
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::{mock_repo, Entity};
    use crate::commands;
    use crate::models::Contact;
    use crate::services::contact_handlers::{Repository, ContactService};

    async fn _create_contact(service: &mut ContactService) -> Result<Contact, String> {
        let cmd = commands::CreateContact {
            name: "Jon Snow".to_string(),
            email: "jon@winterfell.com".to_string(),
        };
        service.create_contact(&cmd).await
    }

    #[actix_web::test]
    async fn can_create_contact() {
        let mut service = ContactService::new(mock_repo());
        let res = _create_contact(&mut service).await;
        let contacts = service.repository.list().unwrap();
        assert_eq!(contacts.len(), 1);
    }

    #[actix_web::test]
    async fn can_update_contact() {
        let mut service = ContactService::new(mock_repo());
        let res = _create_contact(&mut service).await;
        let contacts = service.repository.list().unwrap();
        let id = contacts.first().unwrap().id.clone();

        let contact = service.repository.get(&id);
        assert!(contact.is_some());
        let contact = contact.unwrap();

        let cmd = commands::UpdateContact {
            id: id.clone(),
            name: Some("Arya Stark".to_string()),
            email: None,
        };
        let res = service.update_contact(&cmd).await;
        assert!(res.is_ok());

        let contact = service.repository.get(&id).unwrap();
        assert_eq!(contact.name, "Arya Stark");
    }

    #[actix_web::test]
    async fn can_delete_contact() {
        let mut service = ContactService::new(mock_repo());
        let res = _create_contact(&mut service).await;
        let contacts = service.repository.list().unwrap();
        let id = contacts.first().unwrap().id.clone();

        let res = service.repository.delete(&id);
        assert!(res.is_ok());
        let contacts = service.repository.list().unwrap();
        assert_eq!(contacts.len(), 0);
    }
}
