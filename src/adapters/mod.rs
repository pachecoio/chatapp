mod base;

pub use base::Entity;
pub use base::Repository;

#[cfg(test)]
pub use base::mock_repo;
