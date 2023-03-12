use crate::adapters::{IdType, Repository, RepositoryError};
use crate::models::{Message};
use async_trait::async_trait;

#[async_trait]
pub trait MessageRepository: Repository<Message> {
    async fn get_by_channel_id(
        &self,
        channel_id: &IdType,
        limit: i64,
        offset: u64,
    ) -> Result<Vec<Message>, RepositoryError>;
}
