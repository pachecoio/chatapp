use crate::adapters::{IdType, Model};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: IdType,
    pub name: Option<String>,
    pub channel_type: ChannelType,
    pub contact_ids: Vec<IdType>,
}

impl Model for Channel {
    fn id(&self) -> &IdType {
        &self.id
    }
}

impl Channel {
    pub fn new(name: &str, channel_type: ChannelType, contact_ids: &Vec<IdType>) -> Self {
        Channel {
            id: IdType::String(uuid::Uuid::new_v4().to_string()),
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
