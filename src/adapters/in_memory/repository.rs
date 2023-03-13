use crate::adapters::channel_repository::ChannelRepository;
use crate::adapters::contact_repository::ContactRepository;
use crate::adapters::message_repository::MessageRepository;
use crate::adapters::{IdType, Model, Repository, RepositoryError};
use crate::models::{Channel, Contact, Message};
use async_trait::async_trait;

pub struct InMemoryRepository<M> {
    pub entities: Vec<M>,
}

#[async_trait]
impl<M: Model> Repository<M> for InMemoryRepository<M> {
    async fn create(&mut self, entity: &M) -> Result<M, RepositoryError> {
        self.entities.push(entity.clone());
        Ok(entity.clone())
    }

    async fn update(&mut self, entity: &M) -> Result<(), RepositoryError> {
        let contact = match self.get(&entity.id()).await {
            Some(c) => c,
            None => {
                return Err(RepositoryError {
                    message: "Entity not found".to_string(),
                })
            }
        };
        let index = self
            .entities
            .iter()
            .position(|c| c.id() == contact.id())
            .unwrap();
        self.entities[index] = entity.clone();
        Ok(())
    }

    async fn delete(&mut self, id: &IdType) -> Result<(), RepositoryError> {
        let contact = match self.get(id).await {
            Some(c) => c,
            None => {
                return Err(RepositoryError {
                    message: "Entity not found".to_string(),
                })
            }
        };
        let index = self
            .entities
            .iter()
            .position(|c| c.id() == contact.id())
            .unwrap();
        self.entities.remove(index);
        Ok(())
    }

    async fn get(&self, _id: &IdType) -> Option<M> {
        for entity in self.entities.iter() {
            let id = entity.id();
            if &id == _id {
                return Some(entity.clone());
            }
        }
        None
    }
    async fn list(&self) -> Result<Vec<M>, RepositoryError> {
        Ok(self.entities.clone())
    }
}

#[async_trait]
impl ChannelRepository for InMemoryRepository<Channel> {
    async fn find_by_contact_id(
        &self,
        contact_id: &IdType,
    ) -> Result<Vec<Channel>, RepositoryError> {
        let mut channels = Vec::new();
        for channel in self.entities.iter() {
            if channel.contact_ids.contains(contact_id) {
                channels.push(channel.clone());
            }
        }
        Ok(channels)
    }

    async fn get_by_contact_ids(&self, contact_ids: &Vec<IdType>) -> Option<Channel> {
        for channel in self.entities.iter() {
            if channel.contact_ids.clone().sort() == contact_ids.clone().sort() {
                return Some(channel.clone());
            }
        }
        None
    }
}

#[async_trait]
impl ContactRepository for InMemoryRepository<Contact> {
    async fn find_by_email(&self, email: &str) -> Option<Contact> {
        for contact in self.entities.iter() {
            if contact.email == email {
                return Some(contact.clone());
            }
        }
        None
    }
}

#[async_trait]
impl MessageRepository for InMemoryRepository<Message> {
    async fn get_by_channel_id(
        &self,
        channel_id: &IdType,
        _limit: i64,
        _offset: u64,
    ) -> Result<Vec<Message>, RepositoryError> {
        let mut messages = Vec::new();
        for message in self.entities.iter() {
            if message.channel_id == *channel_id {
                messages.push(message.clone());
            }
        }
        Ok(messages)
    }
}

pub fn mock_message_repo() -> InMemoryRepository<Message> {
    InMemoryRepository { entities: vec![] }
}

pub fn mock_channel_repo() -> InMemoryRepository<Channel> {
    InMemoryRepository { entities: vec![] }
}

/// Creates an in-memory repository with base methods implemented
pub fn mock_repo<M: Model>() -> impl Repository<M> {
    InMemoryRepository { entities: vec![] }
}

pub fn mock_contact_repo() -> InMemoryRepository<Contact> {
    InMemoryRepository { entities: vec![] }
}
