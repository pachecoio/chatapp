use crate::adapters::{IdType, Model};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use mongodb::bson::oid::ObjectId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub _id: Option<ObjectId>,
    pub name: Option<String>,
    pub channel_type: ChannelType,
    pub contact_ids: Vec<IdType>,
}

impl Model for Channel {
    fn id(&self) -> IdType {
        IdType::ObjectId(self._id.clone().unwrap())
    }
}

impl Channel {
    pub fn new(name: &str, channel_type: ChannelType, contact_ids: &Vec<IdType>) -> Self {
        Channel {
            _id: Some(
                ObjectId::new()
            ),
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
