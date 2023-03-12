use crate::adapters::{IdType, Model};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub channel_id: IdType,
    /// The id of the contact that sent the message
    pub from: IdType,
    /// The id of the contact that received the message
    pub to: IdType,
    pub content: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Model for Message {
    fn id(&self) -> IdType {
        IdType::ObjectId(self.id.unwrap())
    }
}

impl Message {
    pub fn new(channel_id: &IdType, from: &IdType, to: &IdType, content: &str) -> Self {
        Message {
            id: Some(ObjectId::new()),
            channel_id: channel_id.clone(),
            from: from.clone(),
            to: to.clone(),
            content: content.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

pub trait DateUtils {
    fn now() -> DateTime<Utc>;
    fn to_timestamp(&self) -> i64;
    fn from_timestamp(timestamp: i64) -> DateTime<Utc>;
}

impl DateUtils for DateTime<Utc> {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn to_timestamp(&self) -> i64 {
        self.timestamp()
    }

    fn from_timestamp(timestamp: i64) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(timestamp, 0), Utc)
    }
}
