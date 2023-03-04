use crate::adapters::Repository;
use crate::commands;
use crate::models::Channel;

pub async fn create_channel<R: Repository<Channel>>(repo: &mut R, cmd: &commands::CreateChannel) -> Result<Channel, ChannelError> {
    let channel = Channel::new(
        &cmd.name, cmd.channel_type.clone()
    );
    match repo.create(&channel) {
        Ok(c) => Ok(c),
        Err(e) => {
            Err(ChannelError {
                message: e.to_string()
            })
        }
    }
}

#[derive(Debug)]
pub struct ChannelError {
    pub message: String,
}

impl std::fmt::Display for ChannelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::{mock_repo, Repository};
    use crate::commands;
    use crate::models::Channel;
    use crate::models::channel::ChannelType;
    use crate::services::channel_handlers::create_channel;

    #[actix_web::test]
    async fn can_create_private_channel() {
        let mut repo = mock_repo();
        let cmd = commands::CreateChannel {
            name: "Test channel".to_string(),
            channel_type: ChannelType::Private,
        };
        let res = create_channel(&mut repo, &cmd).await;
        assert!(res.is_ok());
        let channel = res.unwrap();
        assert_eq!(channel.name, Some("Test channel".to_string()));
        assert_eq!(channel.channel_type, ChannelType::Private);
    }
}
