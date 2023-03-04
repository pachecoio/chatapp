use crate::adapters::ChannelRepository;
use crate::models::Channel;
use std::fmt::Debug;

pub trait Entity: Clone + Debug {
    fn id(&self) -> &str;
}

pub trait Repository<E: Entity> {
    fn create(&mut self, entity: &E) -> Result<E, String>;
    fn update(&mut self, entity: &E) -> Result<(), String>;
    fn delete(&mut self, id: &str) -> Result<(), String>;
    fn get(&self, id: &str) -> Option<E>;
    fn list(&self) -> Result<Vec<E>, String>;
}

#[cfg(test)]
pub struct InMemoryRepository<E> {
    pub(crate) entities: Vec<E>,
}

#[cfg(test)]
impl<E: Entity> Repository<E> for InMemoryRepository<E> {
    fn create(&mut self, entity: &E) -> Result<E, String> {
        self.entities.push(entity.clone());
        Ok(entity.clone())
    }

    fn update(&mut self, entity: &E) -> Result<(), String> {
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

    fn get(&self, id: &str) -> Option<E> {
        for entity in self.entities.iter() {
            let id = entity.id();
            if id == id {
                return Some(entity.clone());
            }
        }
        None
    }
    fn list(&self) -> Result<Vec<E>, String> {
        Ok(self.entities.clone())
    }
}

/// Creates an in-memory repository with base methods implemented
#[cfg(test)]
pub fn mock_repo<E: Entity>() -> impl Repository<E> {
    InMemoryRepository { entities: vec![] }
}
