use crate::adapters::Repository;
use crate::commands;
use crate::models::Contact;

pub async fn create_contact<R: Repository<Contact>>(repo: &mut R, cmd: &commands::CreateContact) -> Result<Contact, String> {
    let contact = Contact {
        id: uuid::Uuid::new_v4().to_string(),
        name: cmd.name.clone(),
        email: cmd.email.clone(),
    };
    repo.create(&contact)?;

    Ok(contact)
}

pub async fn update_contact<R: Repository<Contact>>(repo: &mut R, cmd: &commands::UpdateContact) -> Result<Contact, String> {
    let mut contact = repo.get(&cmd.id).ok_or("Contact not found")?;
    if let Some(name) = &cmd.name {
        contact.name = name.clone();
    }
    if let Some(email) = &cmd.email {
        contact.email = email.clone();
    }
    repo.update(&contact)?;
    Ok(contact)
}

#[cfg(test)]
mod tests {
    use crate::adapters::{Entity, mock_repo};
    use crate::commands;
    use crate::models::Contact;
    use crate::services::contact_handlers::{create_contact, Repository, update_contact};

    async fn _create_contact<R: Repository<Contact>>(repo: &mut R) -> Result<Contact, String> {
        let cmd = commands::CreateContact {
            name: "Jon Snow".to_string(),
            email: "jon@winterfell.com".to_string(),
        };
        create_contact(repo, &cmd).await
    }

    #[actix_web::test]
    async fn can_create_contact() {
        let mut repo = mock_repo();
        let res = _create_contact(&mut repo).await;
        let contacts = repo.list().unwrap();
        assert_eq!(contacts.len(), 1);
    }

    #[actix_web::test]
    async fn can_update_contact() {
        let mut repo = mock_repo();
        let res = _create_contact(&mut repo).await;
        let contacts = repo.list().unwrap();
        let id = contacts.first().unwrap().id.clone();

        let contact = repo.get(&id);
        assert!(contact.is_some());
        let contact = contact.unwrap();

        let cmd = commands::UpdateContact {
            id: id.clone(),
            name: Some("Arya Stark".to_string()),
            email: None
        };
        let res = update_contact(&mut repo, &cmd).await;
        assert!(res.is_ok());

        let contact = repo.get(&id).unwrap();
        assert_eq!(contact.name, "Arya Stark");

    }

    #[actix_web::test]
    async fn can_delete_contact() {
        let mut repo = mock_repo();
        let res = _create_contact(&mut repo).await;
        let contacts = repo.list().unwrap();
        let id = contacts.first().unwrap().id.clone();

        let res = repo.delete(&id);
        assert!(res.is_ok());
        let contacts = repo.list().unwrap();
        assert_eq!(contacts.len(), 0);
    }
}