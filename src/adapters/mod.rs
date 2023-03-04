mod base;
mod channel_repository;

pub use base::Entity;
pub use base::Repository;

#[cfg(test)]
pub use channel_repository::{
    ChannelRepository,
    mock_channel_repo,
};
pub use base::mock_repo;

