use async_trait::async_trait;

use mongodb::bson::oid::ObjectId;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub trait Model: Clone + Debug + Send + Sync + Serialize + DeserializeOwned {
    fn id(&self) -> IdType;
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum IdType {
    String(String),
    ObjectId(ObjectId),
}

impl Display for IdType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IdType::String(s) => write!(f, "{s}"),
            IdType::ObjectId(o) => write!(f, "{o}"),
        }
    }
}

#[async_trait]
pub trait Repository<M: Model> {
    async fn create(&mut self, entity: &M) -> Result<M, RepositoryError>;
    async fn update(&mut self, entity: &M) -> Result<(), RepositoryError>;
    async fn delete(&mut self, id: &IdType) -> Result<(), RepositoryError>;
    async fn get(&self, id: &IdType) -> Option<M>;
    async fn list(
        &self,
        skip: Option<u64>,
        limit: Option<i32>,
    ) -> Result<(i32, Vec<M>), RepositoryError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryError {
    pub message: String,
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
