use crate::adapters::{IdType, Repository, RepositoryError};
use crate::models::Channel;
use async_trait::async_trait;

#[async_trait]
pub trait ChannelRepository: Repository<Channel> {
    async fn find_by_contact_id(
        &self,
        contact_id: &IdType,
    ) -> Result<Vec<Channel>, RepositoryError>;
    async fn get_by_contact_ids(&self, contact_ids: &Vec<IdType>) -> Option<Channel>;
}
