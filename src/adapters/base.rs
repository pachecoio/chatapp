use crate::adapters::ChannelRepository;
use crate::models::Channel;
use std::fmt::Debug;
use async_trait::async_trait;

pub trait Model: Clone + Debug + Send + Sync {
    fn id(&self) -> &str;
}

#[async_trait]
pub trait Repository<M: Model> {
    async fn create(&mut self, entity: &M) -> Result<M, String>;
    async fn update(&mut self, entity: &M) -> Result<(), String>;
    async fn delete(&mut self, id: &str) -> Result<(), String>;
    async fn get(&self, id: &str) -> Option<M>;
    async fn list(&self) -> Result<Vec<M>, String>;
}

pub struct MongoRepository<M> {
    pub collection: mongodb::Collection<M>,
}

impl<M: Model> MongoRepository<M> {
    pub fn new(db: &mongodb::Database, collection_name: &str) -> Self {
        MongoRepository {
            collection: db.collection(collection_name)
        }
    }
}

#[async_trait]
impl<M: Model> Repository<M> for MongoRepository<M> {
    async fn create(&mut self, model: &M) -> Result<M, String> {
        todo!()
    }

    async fn update(&mut self, model: &M) -> Result<(), String> {
        todo!()
    }

    async fn delete(&mut self, id: &str) -> Result<(), String> {
        todo!()
    }

    async fn get(&self, id: &str) -> Option<M> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<M>, String> {
        todo!()
    }
}

#[cfg(test)]
pub struct InMemoryRepository<M> {
    pub entities: Vec<M>,
}

#[cfg(test)]
#[async_trait]
impl<M: Model> Repository<M> for InMemoryRepository<M> {
    async fn create(&mut self, entity: &M) -> Result<M, String> {
        self.entities.push(entity.clone());
        Ok(entity.clone())
    }

    async fn update(&mut self, entity: &M) -> Result<(), String> {
        let contact = self.get(entity.id()).await.ok_or("Entity not found")?;
        let index = self
            .entities
            .iter()
            .position(|c| c.id() == contact.id())
            .unwrap();
        self.entities[index] = entity.clone();
        Ok(())
    }

    async fn delete(&mut self, id: &str) -> Result<(), String> {
        let contact = self.get(id).await.ok_or("Entity not found")?;
        let index = self
            .entities
            .iter()
            .position(|c| c.id() == contact.id())
            .unwrap();
        self.entities.remove(index);
        Ok(())
    }

    async fn get(&self, id: &str) -> Option<M> {
        for entity in self.entities.iter() {
            let id = entity.id();
            if id == id {
                return Some(entity.clone());
            }
        }
        None
    }
    async fn list(&self) -> Result<Vec<M>, String> {
        Ok(self.entities.clone())
    }
}

/// Creates an in-memory repository with base methods implemented
#[cfg(test)]
pub fn mock_repo<M: Model>() -> impl Repository<M> {
    InMemoryRepository { entities: vec![] }
}
