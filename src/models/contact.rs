use crate::adapters::Entity;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl Entity for Contact {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Contact {
    pub fn new(name: &str, email: &str) -> Self {
        Contact {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}
