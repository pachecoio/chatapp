use crate::adapters::{Model, Repository};
use crate::adapters::in_memory::repository::InMemoryRepository;
use crate::models::Contact;
#[cfg(test)]

pub trait ContactRepository: Repository<Contact> {}
