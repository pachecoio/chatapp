use crate::adapters::Repository;
use crate::models::Contact;
use async_trait::async_trait;

#[async_trait]
pub trait ContactRepository: Repository<Contact> {
    async fn find_by_email(&self, email: &str) -> Option<Contact>;
}
