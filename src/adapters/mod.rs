mod base;
pub use base::{IdType, Model, Repository, RepositoryError};

pub mod channel_repository;
pub mod contact_repository;
pub mod mongo;

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
