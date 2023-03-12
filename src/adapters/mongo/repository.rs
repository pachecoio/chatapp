use crate::adapters::channel_repository::ChannelRepository;
use crate::adapters::contact_repository::ContactRepository;
use crate::adapters::{IdType, Model, Repository, RepositoryError};
use crate::models::{Channel, Contact, Message};
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use serde::de::DeserializeOwned;
use crate::adapters::message_repository::MessageRepository;

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

#[async_trait]
impl ChannelRepository for MongoRepository<Channel> {
    async fn get_by_contact_ids(&self, contact_ids: &Vec<IdType>) -> Option<Channel> {
        let ids = contact_ids.iter().map(|id| {
            match id {
                IdType::String(s) => mongodb::bson::oid::ObjectId::parse_str(s).unwrap(),
                IdType::ObjectId(o) => o.clone(),
            }
        }).map(|id|
            doc! {
                "ObjectId": id
            }
        ).collect::<Vec<Document>>();

        self.collection
            .find_one(
                Some(doc! {
                    "contact_ids": {
                        "$all": ids
                        }
                }),
                None,
            )
            .await
            .unwrap()
    }
}

#[async_trait]
impl MessageRepository for MongoRepository<Message> {
    async fn get_by_channel_id(&self, channel_id: &IdType, limit: i64, offset: u64) -> Result<Vec<Message>, RepositoryError> {
        let object_id = match channel_id {
            IdType::String(s) => mongodb::bson::oid::ObjectId::parse_str(s).unwrap(),
            IdType::ObjectId(o) => o.clone(),
        };
        let options = mongodb::options::FindOptions::builder()
            .limit(Some(limit))
            .skip(Some(offset))
            .sort(Some(doc! { "created_at": -1 }))
            .build();

        let mut cursor = self
            .collection
            .find(Some(doc! {
                "channel_id": {
                    "ObjectId": object_id
                },
            }), options)
            .await
            .unwrap();
        let mut messages = Vec::new();
        while let Some(result) = cursor.try_next().await.unwrap() {
            messages.push(result);
        }
        Ok(messages)
    }
}
