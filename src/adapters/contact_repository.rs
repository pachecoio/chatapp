use crate::adapters::base::InMemoryRepository;
use crate::adapters::Repository;
use crate::models::Contact;

pub trait ContactRepository: Repository<Contact> {}

#[cfg(test)]
impl ContactRepository for InMemoryRepository<Contact> {}

#[cfg(test)]
pub fn mock_contact_repo() -> InMemoryRepository<Contact> {
    InMemoryRepository { entities: vec![] }
}
