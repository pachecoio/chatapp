use crate::adapters::channel_repository::ChannelRepository;
use crate::adapters::contact_repository::ContactRepository;
use crate::adapters::Repository;
use crate::commands;
use crate::models::{Channel, ChannelType};

pub struct ChannelService<'a> {
    repository: &'a mut dyn ChannelRepository,
    contact_repository: &'a mut dyn ContactRepository,
}

impl<'a> ChannelService<'a> {
    // pub fn new<R: Repository<Channel> + 'static, C: Repository<Contact> + 'static>(
    //     repository: R,
    //     contact_repository: C,
    // ) -> Self {
    //     ChannelService {
    //         repository: Box::new(repository),
    //         contact_repository: Box::new(contact_repository),
    //     }
    // }

    fn new(
        repo: &'a mut dyn ChannelRepository,
        contact_repository: &'a mut dyn ContactRepository,
    ) -> Self {
        ChannelService {
            repository: repo,
            contact_repository,
        }
    }

    pub async fn create_channel(
        &mut self,
        cmd: &commands::CreateChannel,
    ) -> Result<Channel, ChannelError> {
        validate_channel(cmd)?;
        let channel = Channel::new(&cmd.name, cmd.channel_type.clone(), &cmd.contact_ids);
        match self.repository.create(&channel).await {
            Ok(c) => Ok(c),
            Err(e) => Err(ChannelError {
                message: e.to_string(),
            }),
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
    use crate::adapters::{
        mock_channel_repo, mock_contact_repo, mock_repo, Model, Repository,
    };
    use crate::commands;
    use crate::models::{ChannelType, Contact};
    use crate::services::channel_handlers::ChannelService;

    /// Creates a mock repository with two contacts
    async fn mock_repo_with_contacts<M: Model>() -> (impl Repository<Contact>, Vec<Contact>) {
        let mut contact_repo = mock_repo();
        let jon = contact_repo
            .create(&Contact::new("Jon Snow", "jon@winterfell.com"))
            .await
            .unwrap();
        let arya = contact_repo
            .create(&Contact::new("Arya Stark", "arya@winterfell.com"))
            .await
            .unwrap();
        (contact_repo, vec![jon, arya])
    }

    async fn add_mock_contacts(repo: &mut impl Repository<Contact>) -> Vec<Contact> {
        let jon = repo
            .create(&Contact::new("Jon Snow", "jon@winterfell.com"))
            .await
            .unwrap();
        let arya = repo
            .create(&Contact::new("Arya Stark", "arya@winterfell.com"))
            .await
            .unwrap();
        vec![jon, arya]
    }

    #[actix_web::test]
    async fn create_private_channel() {
        let mut repo = mock_channel_repo();
        let mut c_repo = mock_contact_repo();
        let contacts = add_mock_contacts(&mut c_repo).await;
        let mut service = ChannelService::new(&mut repo, &mut c_repo);
        let cmd = commands::CreateChannel {
            name: "Private channel".to_string(),
            channel_type: ChannelType::Private,
            contact_ids: contacts.iter().map(|c| c.id()).collect(),
        };
        let res = service.create_channel(&cmd).await;
        assert!(res.is_ok());
        let channel = res.unwrap();
        assert_eq!(channel.contact_ids.len(), 2);
    }

    #[actix_web::test]
    async fn cannot_create_private_channel_with_less_than_two_contacts() {
        let mut repo = mock_channel_repo();
        let mut c_repo = mock_contact_repo();
        let mut service = ChannelService::new(&mut repo, &mut c_repo);
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
        let mut repo = mock_channel_repo();
        let mut c_repo = mock_contact_repo();
        let contacts = add_mock_contacts(&mut c_repo).await;
        let mut service = ChannelService::new(&mut repo, &mut c_repo);
        let cmd = commands::CreateChannel {
            name: "Group channel".to_string(),
            channel_type: ChannelType::Group,
            contact_ids: contacts.iter().map(|c| c.id()).collect(),
        };
        let res = service.create_channel(&cmd).await;
        assert!(res.is_ok());
        let channel = res.unwrap();
        assert_eq!(channel.contact_ids.len(), 2);
        assert_eq!(channel.channel_type, ChannelType::Group);
    }
}

#[cfg(test)]
mod tests_mongo {
    use crate::adapters::mongo::repository::MongoRepository;
    use crate::adapters::{Model, Repository};
    use crate::models::Contact;
    use crate::services::ChannelService;

    async fn add_mock_contacts(repo: &mut impl Repository<Contact>) -> Vec<Contact> {
        let jon = repo
            .create(&Contact::new("Jon Snow", "jon@winterfell.com"))
            .await
            .unwrap();
        let arya = repo
            .create(&Contact::new("Arya Stark", "arya@winterfell.com"))
            .await
            .unwrap();
        vec![jon, arya]
    }

    #[actix_web::test]
    async fn create_channel() {
        let db = crate::adapters::mongo::database::init("test").await;
        let mut repo = MongoRepository::new(&db, "channels");
        let mut c_repo = MongoRepository::new(&db, "contacts");
        let contacts = add_mock_contacts(&mut c_repo).await;

        let mut service = ChannelService::new(&mut repo, &mut c_repo);
        let cmd = crate::commands::CreateChannel {
            name: "Private channel".to_string(),
            channel_type: crate::models::ChannelType::Private,
            contact_ids: contacts.iter().map(|c| c.id()).collect(),
        };
        let res = service.create_channel(&cmd).await;
        assert!(res.is_ok());

        let mut channel = res.unwrap();
        assert_eq!(channel.contact_ids.len(), 2);

        let channels = service.repository.list().await.unwrap();
        assert!(!channels.is_empty());

        channel.name = Some("Updated channel".to_string());

        let updated = service.repository.update(&channel).await;
        assert!(updated.is_ok());

        let channel = service.repository.get(&channel.id()).await.unwrap();
        assert_eq!(channel.name.clone().unwrap(), "Updated channel");

        let res = repo.delete(&channel.id()).await;
        assert!(res.is_ok());
    }
}
