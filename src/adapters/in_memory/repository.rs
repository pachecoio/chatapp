use crate::adapters::{Model, Repository, RepositoryError};
use crate::models::{Channel, Contact};
use async_trait::async_trait;
use crate::adapters::channel_repository::ChannelRepository;
use crate::adapters::contact_repository::ContactRepository;

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
        let contact = match self.get(entity.id()).await {
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

    async fn delete(&mut self, id: &str) -> Result<(), RepositoryError> {
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

    async fn get(&self, _id: &str) -> Option<M> {
        for entity in self.entities.iter() {
            let id = entity.id();
            if id == id {
                return Some(entity.clone());
            }
        }
        None
    }
    async fn list(&self) -> Result<Vec<M>, RepositoryError> {
        Ok(self.entities.clone())
    }
}

impl ChannelRepository for InMemoryRepository<Channel> {
    fn get_by_contact_ids(&self, contact_ids: &Vec<String>) -> Option<Channel> {
        for channel in self.entities.iter() {
            if channel.contact_ids.clone().sort() == contact_ids.clone().sort() {
                return Some(channel.clone());
            }
        }
        None
    }
}

impl ContactRepository for InMemoryRepository<Contact> {}

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
