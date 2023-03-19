use crate::adapters::IdType;
use crate::models::ChannelType;
use serde::{Deserialize, Serialize};

pub struct SendMessage {
    pub channel_id: Option<IdType>,
    pub from: IdType,
    pub to: IdType,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContact {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContact {
    pub id: IdType,
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct CreateChannel {
    pub name: String,
    pub channel_type: ChannelType,
    pub contact_ids: Vec<IdType>,
}
