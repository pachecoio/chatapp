use crate::adapters::Entity;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: Option<String>,
    pub channel_type: ChannelType,
    pub contact_ids: Vec<String>,
}

impl Entity for Channel {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Channel {
    pub fn new(name: &str, channel_type: ChannelType, contact_ids: &Vec<String>) -> Self {
        Channel {
            id: uuid::Uuid::new_v4().to_string(),
            name: Some(name.to_string()),
            channel_type,
            contact_ids: contact_ids.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ChannelType {
    Private,
    Group,
}
