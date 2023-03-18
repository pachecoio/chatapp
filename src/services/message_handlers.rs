use crate::adapters::{IdType, Model, Repository};
use crate::commands;

use crate::models::{Channel, ChannelType, Contact, Message};

use crate::adapters::channel_repository::ChannelRepository;
use crate::adapters::contact_repository::ContactRepository;
use crate::adapters::message_repository::MessageRepository;
use std::fmt::{Display, Formatter};

pub struct MessageService<'a> {
    repository: &'a mut dyn MessageRepository,
    channel_repository: &'a mut dyn ChannelRepository,
    contact_repository: &'a mut dyn ContactRepository,
}

impl<'a> MessageService<'a> {
    pub fn new(
        repo: &'a mut dyn MessageRepository,
        channel_repository: &'a mut dyn ChannelRepository,
        contact_repository: &'a mut dyn ContactRepository,
    ) -> Self {
        MessageService {
            repository: repo,
            channel_repository,
            contact_repository,
        }
    }

    pub async fn send_message(&mut self, cmd: &commands::SendMessage) -> Result<(), MessageError> {
        let contact_from = self.get_contact(&cmd.from).await?;
        let contact_to = self.get_contact(&cmd.to).await?;
        let channel = match &cmd.channel_id {
            None => {
                self.create_private_channel(&vec![contact_from.id(), contact_to.id()])
                    .await?
            }
            Some(c) => self.get_channel(c).await?,
        };
        let message = Message::new(&channel.id(), &cmd.from, &cmd.to, &cmd.content);
        match self.repository.create(&message).await {
            Ok(_) => Ok(()),
            Err(e) => Err(MessageError {
                message: e.to_string(),
            }),
        }
    }

    async fn get_contact(&mut self, id: &IdType) -> Result<Contact, MessageError> {
        match self.contact_repository.get(id).await {
            None => Err(MessageError {
                message: format!("Contact with id {id} not found"),
            }),
            Some(c) => Ok(c),
        }
    }

    async fn get_channel(&mut self, id: &IdType) -> Result<Channel, MessageError> {
        match self.channel_repository.get(id).await {
            None => Err(MessageError {
                message: format!("Channel with id {id} not found"),
            }),
            Some(c) => Ok(c),
        }
    }

    async fn create_private_channel(
        &mut self,
        contact_ids: &Vec<IdType>,
    ) -> Result<Channel, MessageError> {
        match self
            .channel_repository
            .get_by_contact_ids(contact_ids)
            .await
        {
            /// Returns channel if already exists
            Some(c) => Ok(c),
            /// Creates a new channel if it doesn't exist
            None => {
                let channel = Channel::new("", ChannelType::Private, contact_ids);
                match self.channel_repository.create(&channel).await {
                    Ok(c) => Ok(c),
                    Err(e) => Err(MessageError {
                        message: e.to_string(),
                    }),
                }
            }
        }
    }

    async fn get_messages(&mut self, channel_id: &IdType) -> Result<Vec<Message>, MessageError> {
        match self.repository.get_by_channel_id(channel_id, 100, 0).await {
            Ok(m) => Ok(m),
            Err(e) => Err(MessageError {
                message: e.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct MessageError {
    pub message: String,
}

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(test)]
async fn add_test_contacts(repo: &mut impl Repository<Contact>) -> Vec<Contact> {
    let c1 = repo
        .create(&Contact::new("Sansa Stark", "sansa@winterfell.com"))
        .await
        .unwrap();
    let c2 = repo
        .create(&Contact::new("Eddard Stark", "eddard@winterfell.com"))
        .await
        .unwrap();
    vec![c1, c2]
}

#[cfg(test)]
async fn add_test_channel(repo: &mut impl Repository<Channel>, contacts: &[Contact]) -> Channel {
    let cmd = commands::CreateChannel {
        name: "The North Remembers".to_string(),
        channel_type: ChannelType::Private,
        contact_ids: vec![contacts[0].id(), contacts[1].id()],
    };
    repo.create(&Channel::new(
        &cmd.name,
        cmd.channel_type.clone(),
        &cmd.contact_ids,
    ))
    .await
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::{mock_channel_repo, mock_contact_repo, mock_message_repo, Model};

    #[actix_web::test]
    async fn can_send_message() {
        let mut repo = mock_message_repo();
        let mut contact_repo = mock_contact_repo();
        let mut channel_repo = mock_channel_repo();

        let contacts = add_test_contacts(&mut contact_repo).await;
        let _channel = add_test_channel(&mut channel_repo, &contacts).await;

        let mut service = MessageService::new(&mut repo, &mut channel_repo, &mut contact_repo);

        let cmd = commands::SendMessage {
            channel_id: None,
            from: contacts[0].id(),
            to: contacts[1].id(),
            content: "The north remembers!".to_string(),
        };
        let res = service.send_message(&cmd).await;

        let (_total, channels) = service.channel_repository.list(None, None).await.unwrap();
        assert_eq!(channels.len(), 1, "Should not have created a new channel");
        assert!(res.is_ok());

        let messages = service.get_messages(&channels[0].id()).await.unwrap();
        assert_eq!(messages.len(), 1, "Should have created a new message");
        let message = messages.first().unwrap();
        assert_eq!(message.from, cmd.from);
        assert_eq!(message.to, cmd.to);
        assert_eq!(message.content, cmd.content);
    }
}

#[cfg(test)]
mod tests_mongo {
    use crate::adapters::channel_repository::ChannelRepository;
    use crate::adapters::mongo::repository::MongoRepository;
    use crate::adapters::{Model, Repository};
    use crate::commands;
    use crate::services::message_handlers::{add_test_channel, add_test_contacts, MessageService};

    #[actix_web::test]
    #[ignore]
    async fn can_send_message() {
        let db = crate::adapters::mongo::database::init("test").await;
        let mut repo = MongoRepository::new(&db, "messages");
        let mut contacts_repo = MongoRepository::new(&db, "contacts");
        let mut channels_repo = MongoRepository::new(&db, "channels");
        let contacts = add_test_contacts(&mut contacts_repo).await;
        let channel = add_test_channel(&mut channels_repo, &contacts).await;

        let mut service = MessageService::new(&mut repo, &mut channels_repo, &mut contacts_repo);

        let cmd = commands::SendMessage {
            channel_id: None,
            from: contacts[0].id(),
            to: contacts[1].id(),
            content: "The north remembers!".to_string(),
        };
        let res = service.send_message(&cmd).await;
        assert!(res.is_ok());

        let messages = service.get_messages(&channel.id()).await.unwrap();
        assert!(!messages.is_empty(), "Should have created a new message");

        // cleanup
        for c in contacts {
            contacts_repo.delete(&c.id()).await.unwrap();
        }
        channels_repo.delete(&channel.id()).await.unwrap();
        for m in messages {
            repo.delete(&m.id()).await.unwrap();
        }
    }

    #[actix_web::test]
    #[ignore]
    async fn find_by_contact_ids() {
        let db = crate::adapters::mongo::database::init("test").await;
        let mut repo = MongoRepository::new(&db, "channels");
        let mut contacts_repo = MongoRepository::new(&db, "contacts");
        let contacts = add_test_contacts(&mut contacts_repo).await;
        let test_channel = add_test_channel(&mut repo, &contacts).await;

        let channel = repo
            .get_by_contact_ids(&vec![contacts[0].id(), contacts[1].id()])
            .await
            .unwrap();
        assert_eq!(channel.id(), test_channel.id());

        // cleanup
        for c in contacts {
            contacts_repo.delete(&c.id()).await.unwrap();
        }
        repo.delete(&test_channel.id()).await.unwrap();
    }
}
