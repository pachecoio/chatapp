use crate::adapters::Repository;
use crate::commands;
use crate::models::{Channel, ChannelType, Contact};

pub struct ChannelService {
    repository: Box<dyn Repository<Channel>>,
    contact_repository: Box<dyn Repository<Contact>>,
}

impl ChannelService {
    pub fn new<R: Repository<Channel> + 'static, C: Repository<Contact> + 'static>(
        repository: R,
        contact_repository: C,
    ) -> Self {
        ChannelService {
            repository: Box::new(repository),
            contact_repository: Box::new(contact_repository),
        }
    }

    pub async fn create_channel(
        &mut self,
        cmd: &commands::CreateChannel,
    ) -> Result<Channel, ChannelError> {
        validate_channel(cmd)?;
        let channel = Channel::new(&cmd.name, cmd.channel_type.clone(), &cmd.contact_ids);
        match self.repository.create(&channel) {
            Ok(c) => Ok(c),
            Err(e) => Err(ChannelError { message: e }),
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

fn validate_channel(cmd: &commands::CreateChannel) -> Result<(), ChannelError> {
    match cmd.channel_type {
        ChannelType::Private => validate_private_channel(cmd),
        _ => Ok(()),
    }
}
fn validate_private_channel(cmd: &commands::CreateChannel) -> Result<(), ChannelError> {
    if cmd.contact_ids.len() != 2 {
        Err(ChannelError {
            message: "Private channels must have exactly 2 contacts".to_string(),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::{mock_repo, Entity, Repository};
    use crate::commands;
    use crate::models::{Channel, ChannelType, Contact};
    use crate::services::channel_handlers::ChannelService;

    /// Creates a mock repository with two contacts
    async fn mock_repo_with_contacts<E: Entity>() -> (impl Repository<Contact>, Vec<Contact>) {
        let mut contact_repo = mock_repo();
        let jon = contact_repo
            .create(&Contact::new("Jon Snow", "jon@winterfell.com"))
            .unwrap();
        let arya = contact_repo
            .create(&Contact::new("Arya Stark", "arya@winterfell.com"))
            .unwrap();
        (contact_repo, vec![jon, arya])
    }

    #[actix_web::test]
    async fn create_private_channel() {
        let (contact_repo, contacts) = mock_repo_with_contacts::<Contact>().await;
        let mut service = ChannelService::new(mock_repo(), contact_repo);
        let cmd = commands::CreateChannel {
            name: "Private channel".to_string(),
            channel_type: ChannelType::Private,
            contact_ids: contacts.iter().map(|c| c.id.clone()).collect(),
        };
        let res = service.create_channel(&cmd).await;
        assert!(res.is_ok());
        let channel = res.unwrap();
        assert_eq!(channel.contact_ids.len(), 2);
    }

    #[actix_web::test]
    async fn cannot_create_private_channel_with_less_than_two_contacts() {
        let mut service = ChannelService::new(mock_repo(), mock_repo());
        let cmd = commands::CreateChannel {
            name: "Private channel without contacts".to_string(),
            channel_type: ChannelType::Private,
            contact_ids: vec![],
        };
        let res = service.create_channel(&cmd).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.message, "Private channels must have exactly 2 contacts");
    }

    #[actix_web::test]
    async fn create_group_channel() {
        let (contact_repo, contacts) = mock_repo_with_contacts::<Contact>().await;
        let mut service = ChannelService::new(mock_repo(), contact_repo);
        let cmd = commands::CreateChannel {
            name: "Group channel".to_string(),
            channel_type: ChannelType::Group,
            contact_ids: contacts.iter().map(|c| c.id.clone()).collect(),
        };
        let res = service.create_channel(&cmd).await;
        assert!(res.is_ok());
        let channel = res.unwrap();
        assert_eq!(channel.contact_ids.len(), 2);
        assert_eq!(channel.channel_type, ChannelType::Group);
    }
}
