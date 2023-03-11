use crate::adapters::Repository;
use crate::models::Contact;

pub trait ContactRepository: Repository<Contact> {}
