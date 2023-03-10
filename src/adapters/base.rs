use crate::adapters::ChannelRepository;
use crate::models::Channel;
use std::fmt::Debug;

pub trait Model: Clone + Debug {
    fn id(&self) -> &str;
}

pub trait Repository<M: Model> {
    fn create(&mut self, entity: &M) -> Result<M, String>;
    fn update(&mut self, entity: &M) -> Result<(), String>;
    fn delete(&mut self, id: &str) -> Result<(), String>;
    fn get(&self, id: &str) -> Option<M>;
    fn list(&self) -> Result<Vec<M>, String>;
}

pub struct MongoRepository<'a> {
    pub db: &'a mongodb::Database,
}

impl<M: Model> Repository<M> for MongoRepository<'_> {
    fn create(&mut self, entity: &M) -> Result<M, String> {
        todo!()
    }

    fn update(&mut self, entity: &M) -> Result<(), String> {
        todo!()
    }

    fn delete(&mut self, id: &str) -> Result<(), String> {
        todo!()
    }

    fn get(&self, id: &str) -> Option<M> {
        todo!()
    }

    fn list(&self) -> Result<Vec<M>, String> {
        todo!()
    }
}

#[cfg(test)]
pub struct InMemoryRepository<M> {
    pub entities: Vec<M>,
}

#[cfg(test)]
impl<M: Model> Repository<M> for InMemoryRepository<M> {
    fn create(&mut self, entity: &M) -> Result<M, String> {
        self.entities.push(entity.clone());
        Ok(entity.clone())
    }

    fn update(&mut self, entity: &M) -> Result<(), String> {
        let contact = self.get(entity.id()).ok_or("Entity not found")?;
        let index = self
            .entities
            .iter()
            .position(|c| c.id() == contact.id())
            .unwrap();
        self.entities[index] = entity.clone();
        Ok(())
    }

    fn delete(&mut self, id: &str) -> Result<(), String> {
        let contact = self.get(id).ok_or("Entity not found")?;
        let index = self
            .entities
            .iter()
            .position(|c| c.id() == contact.id())
            .unwrap();
        self.entities.remove(index);
        Ok(())
    }

    fn get(&self, id: &str) -> Option<M> {
        for entity in self.entities.iter() {
            let id = entity.id();
            if id == id {
                return Some(entity.clone());
            }
        }
        None
    }
    fn list(&self) -> Result<Vec<M>, String> {
        Ok(self.entities.clone())
    }
}

/// Creates an in-memory repository with base methods implemented
#[cfg(test)]
pub fn mock_repo<M: Model>() -> impl Repository<M> {
    InMemoryRepository { entities: vec![] }
}
