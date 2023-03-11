use crate::adapters::contact_repository::ContactRepository;
use crate::adapters::{IdType, Model, Repository, RepositoryError};
use crate::models::Contact;
use async_trait::async_trait;
use futures::TryStreamExt;
use serde::de::DeserializeOwned;

pub struct MongoRepository<M> {
    pub collection: mongodb::Collection<M>,
}

impl<M: Model> MongoRepository<M> {
    pub fn new(db: &mongodb::Database, collection_name: &str) -> Self {
        MongoRepository {
            collection: db.collection(collection_name),
        }
    }
}

#[async_trait]
impl<M> Repository<M> for MongoRepository<M>
where
    M: Model + DeserializeOwned + Unpin + Send + Sync,
{
    async fn create(&mut self, model: &M) -> Result<M, RepositoryError> {
        let result = self.collection.insert_one(model, None).await;
        match result {
            Ok(_) => Ok(model.clone()),
            Err(e) => Err(RepositoryError {
                message: e.to_string(),
            }),
        }
    }

    async fn update(&mut self, _model: &M) -> Result<(), RepositoryError> {
        todo!()
    }

    async fn delete(&mut self, _id: &IdType) -> Result<(), RepositoryError> {
        todo!()
    }

    async fn get(&self, _id: &IdType) -> Option<M> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<M>, RepositoryError> {
        let mut cursor = self.collection.find(None, None).await.unwrap();
        let mut models = Vec::new();

        while let Some(result) = cursor.try_next().await.unwrap() {
            models.push(result);
        }

        Ok(models)
    }
}

impl ContactRepository for MongoRepository<Contact> {}
