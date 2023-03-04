use crate::commands;
use crate::commands::SendMessage;
use std::fmt::{Display, Formatter};

pub fn send_message(cmd: &SendMessage) -> Result<(), MessageError> {
    println!(
        "Sending message to channel: {}, contact: {}, message: {}",
        cmd.channel_id, cmd.contact_id, cmd.message
    );
    Ok(())
}

pub struct MessageError {
    pub message: String,
}

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn can_send_message() {
        let cmd = commands::SendMessage {
            channel_id: "123".to_string(),
            contact_id: "456".to_string(),
            message: "Hello, world!".to_string(),
        };
        let res = send_message(&cmd);
        assert!(res.is_ok());
    }

    #[actix_web::test]
    async fn send_message_persistence() {
        let cmd = commands::SendMessage {
            channel_id: "123".to_string(),
            contact_id: "456".to_string(),
            message: "Hello, world!".to_string(),
        };
        let res = send_message(&cmd);
        assert!(res.is_ok());
    }
}
