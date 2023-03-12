use crate::adapters::{IdType, Model};
use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
}

impl Model for Contact {
    fn id(&self) -> IdType {
        IdType::ObjectId(self._id.clone().unwrap())
    }
}

impl Contact {
    pub fn new(name: &str, email: &str) -> Self {
        Contact {
            _id: Some(
                ObjectId::new()
            ),
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}
