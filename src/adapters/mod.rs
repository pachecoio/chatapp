mod base;
pub use base::{IdType, Model, Repository, RepositoryError};

pub mod channel_repository;
pub mod contact_repository;
pub mod mongo;

#[cfg(test)]
mod in_memory;
pub mod message_repository;

#[cfg(test)]
pub use in_memory::repository::{
    mock_channel_repo, mock_contact_repo, mock_message_repo, mock_repo, InMemoryRepository,
};
