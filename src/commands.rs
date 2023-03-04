use crate::models::channel::ChannelType;

pub struct SendMessage {
    pub channel_id: String,
    pub contact_id: String,
    pub message: String,
}

pub struct CreateContact {
    pub name: String,
    pub email: String,
}

pub struct UpdateContact {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct CreateChannel {
    pub name: String,
    pub channel_type: ChannelType,
    pub contact_ids: Vec<String>,
}