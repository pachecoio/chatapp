use crate::adapters::{IdType, Repository};
use crate::models::Channel;

pub trait ChannelRepository: Repository<Channel> {
    fn get_by_contact_ids(&self, contact_ids: &Vec<IdType>) -> Option<Channel>;
}
