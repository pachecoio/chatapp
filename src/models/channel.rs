use std::fmt::{Debug, Formatter};
use uuid;
use crate::adapters::Entity;

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: String,
    pub name: Option<String>,
    pub channel_type: ChannelType
}

impl Entity for Channel {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Channel {
    pub fn new(name: &str, channel_type: ChannelType) -> Self {
        Channel {
            id: uuid::Uuid::new_v4().to_string(),
            name: Some(name.to_string()),
            channel_type
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChannelType {
    Private,
    Group
}
