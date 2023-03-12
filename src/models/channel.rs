use crate::adapters::{IdType, Model};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub channel_type: ChannelType,
    pub contact_ids: Vec<IdType>,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Model for Channel {
    fn id(&self) -> IdType {
        IdType::ObjectId(self.id.clone().unwrap())
    }
}

impl Channel {
    pub fn new(name: &str, channel_type: ChannelType, contact_ids: &Vec<IdType>) -> Self {
        Channel {
            id: Some(ObjectId::new()),
            name: Some(name.to_string()),
            channel_type,
            contact_ids: contact_ids.to_owned(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ChannelType {
    Private,
    Group,
}
