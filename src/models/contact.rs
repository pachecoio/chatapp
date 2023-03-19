use crate::adapters::{IdType, Model};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Model for Contact {
    fn id(&self) -> IdType {
        IdType::ObjectId(self.id.unwrap())
    }
}

impl Contact {
    pub fn new(name: &str, email: &str) -> Self {
        Contact {
            id: Some(ObjectId::new()),
            name: name.to_string(),
            email: email.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
