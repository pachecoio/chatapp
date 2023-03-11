use mongodb::bson::oid::ObjectId;
use crate::adapters::{IdType, Model};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<ObjectId>,
    pub id: IdType,
    pub name: String,
    pub email: String,
}

impl Model for Contact {
    fn id(&self) -> &IdType {
        &self.id
    }
}

impl Contact {
    pub fn new(name: &str, email: &str) -> Self {
        Contact {
            _id: None,
            id: IdType::String(uuid::Uuid::new_v4().to_string()),
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}
