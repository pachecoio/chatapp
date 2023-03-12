use crate::adapters::channel_repository::ChannelRepository;
use crate::adapters::contact_repository::ContactRepository;
use crate::adapters::{IdType, Model, Repository, RepositoryError};
use crate::models::{Channel, Contact};
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::doc;
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

    async fn update(&mut self, model: &M) -> Result<(), RepositoryError> {
        let doc = match model.id() {
            IdType::String(s) => doc! { "id": s },
            IdType::ObjectId(o) => doc! { "_id": o },
        };
        let result = self.collection.replace_one(doc, model, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RepositoryError {
                message: e.to_string(),
            }),
        }
    }

    async fn delete(&mut self, id: &IdType) -> Result<(), RepositoryError> {
        let doc = match id {
            IdType::String(s) => doc! { "id": s },
            IdType::ObjectId(o) => doc! { "_id": o },
        };
        let result = self.collection.delete_one(doc, None).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RepositoryError {
                message: e.to_string(),
            }),
        }
    }

    async fn get(&self, _id: &IdType) -> Option<M> {
        match _id {
            IdType::String(s) => {
                let result = self
                    .collection
                    .find_one(Some(doc! { "id": s }), None)
                    .await
                    .unwrap();
                result
            }
            IdType::ObjectId(o) => {
                let result = self
                    .collection
                    .find_one(Some(doc! { "_id": o }), None)
                    .await
                    .unwrap();
                result
            }
        }
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

impl ChannelRepository for MongoRepository<Channel> {
    fn get_by_contact_ids(&self, _contact_ids: &Vec<IdType>) -> Option<Channel> {
        todo!()
    }
}
