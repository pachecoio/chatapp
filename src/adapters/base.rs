use async_trait::async_trait;

use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Display, Formatter};
use mongodb::bson::oid::ObjectId;

pub trait Model: Clone + Debug + Send + Sync + Serialize + DeserializeOwned {
    fn id(&self) -> &IdType;
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum IdType {
    String(String),
    ObjectId(ObjectId),
}

impl Display for IdType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IdType::String(s) => write!(f, "{}", s),
            IdType::ObjectId(o) => write!(f, "{}", o),
        }
    }
}

#[async_trait]
pub trait Repository<M: Model> {
    async fn create(&mut self, entity: &M) -> Result<M, RepositoryError>;
    async fn update(&mut self, entity: &M) -> Result<(), RepositoryError>;
    async fn delete(&mut self, id: &IdType) -> Result<(), RepositoryError>;
    async fn get(&self, id: &IdType) -> Option<M>;
    async fn list(&self) -> Result<Vec<M>, RepositoryError>;
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
