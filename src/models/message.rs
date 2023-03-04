use crate::adapters::Entity;
use chrono::{DateTime, Utc};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    /// The id of the contact that sent the message
    pub from: String,
    /// The id of the contact that received the message
    pub to: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Entity for Message {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Message {
    pub fn new(channel_id: &str, from: &str, to: &str, content: &str) -> Self {
        Message {
            id: uuid::Uuid::new_v4().to_string(),
            channel_id: channel_id.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            content: content.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Message", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("channel_id", &self.channel_id)?;
        state.serialize_field("from", &self.from)?;
        state.serialize_field("to", &self.to)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("created_at", &self.created_at.to_timestamp())?;
        state.serialize_field("updated_at", &self.updated_at.to_timestamp())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MessageData {
            id: String,
            channel_id: String,
            from: String,
            to: String,
            content: String,
            created_at: i64,
            updated_at: i64,
        }

        let data = MessageData::deserialize(deserializer)?;
        Ok(Message {
            id: data.id,
            channel_id: data.channel_id,
            from: data.from,
            to: data.to,
            content: data.content,
            created_at: DateTime::<Utc>::from_utc(
                chrono::NaiveDateTime::from_timestamp(data.created_at, 0),
                Utc,
            ),
            updated_at: DateTime::<Utc>::from_utc(
                chrono::NaiveDateTime::from_timestamp(data.updated_at, 0),
                Utc,
            ),
        })
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
