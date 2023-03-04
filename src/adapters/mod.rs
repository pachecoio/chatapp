mod base;

pub use base::Repository;
pub use base::Entity;

#[cfg(test)]
pub use base::mock_repo;