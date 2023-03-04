use crate::adapters::base::InMemoryRepository;
use crate::adapters::Repository;
use crate::models::Channel;

pub trait ChannelRepository: Repository<Channel> {
    fn get_by_contact_ids(&self, contact_ids: &Vec<String>) -> Option<Channel>;
}

#[cfg(test)]
impl ChannelRepository for InMemoryRepository<Channel> {
    fn get_by_contact_ids(&self, contact_ids: &Vec<String>) -> Option<Channel> {
        for channel in self.entities.iter() {
            if channel.contact_ids.clone().sort() == contact_ids.clone().sort() {
                return Some(channel.clone());
            }
        }
        None
    }
}

#[cfg(test)]
pub fn mock_channel_repo() -> InMemoryRepository<Channel> {
    InMemoryRepository { entities: vec![] }
}
