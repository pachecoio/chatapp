mod base;
mod channel_repository;
mod contact_repository;
pub mod mongo;

pub use base::Model;
pub use base::{Repository, RepositoryError};

pub use channel_repository::ChannelRepository;
pub use contact_repository::ContactRepository;

#[cfg(test)]
mod in_memory;

#[cfg(test)]
pub use in_memory::repository::mock_channel_repo;
#[cfg(test)]
pub use in_memory::repository::mock_contact_repo;
#[cfg(test)]
pub use in_memory::repository::mock_repo;
#[cfg(test)]
pub use in_memory::repository::InMemoryRepository;
