use crate::adapters::ChannelRepository;
use crate::models::Channel;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub trait Model: Clone + Debug + Send + Sync + Serialize + DeserializeOwned {
    fn id(&self) -> &str;
}

#[async_trait]
pub trait Repository<M: Model> {
    async fn create(&mut self, entity: &M) -> Result<M, RepositoryError>;
    async fn update(&mut self, entity: &M) -> Result<(), RepositoryError>;
    async fn delete(&mut self, id: &str) -> Result<(), RepositoryError>;
    async fn get(&self, id: &str) -> Option<M>;
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
