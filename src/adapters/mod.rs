mod base;
mod channel_repository;
mod contact_repository;
pub mod database;

pub use base::Model;
pub use base::{
    Repository,
    RepositoryError
};

#[cfg(test)]
pub use base::mock_repo;
pub use channel_repository::ChannelRepository;
pub use contact_repository::ContactRepository;

#[cfg(test)]
pub use channel_repository::mock_channel_repo;
#[cfg(test)]
pub use contact_repository::mock_contact_repo;

pub use base::MongoRepository;