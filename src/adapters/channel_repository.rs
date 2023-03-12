use crate::adapters::{IdType, Repository};
use crate::models::Channel;
use async_trait::async_trait;

#[async_trait]
pub trait ChannelRepository: Repository<Channel> {
    async fn get_by_contact_ids(&self, contact_ids: &Vec<IdType>) -> Option<Channel>;
}
