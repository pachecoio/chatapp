use crate::adapters::{ChannelRepository, ContactRepository, Repository};
use crate::commands;
use crate::commands::SendMessage;
use crate::models::{Channel, ChannelType, Contact, Message};
use std::fmt::{Display, Formatter};
use async_trait::async_trait;

pub struct MessageService {
    repository: Box<dyn Repository<Message>>,
    channel_repository: Box<dyn ChannelRepository>,
    contact_repository: Box<dyn Repository<Contact>>,
}

impl MessageService {
    pub fn new<
        R: Repository<Message> + 'static,
        C: ChannelRepository + 'static,
        CO: ContactRepository + 'static,
    >(
        repository: R,
        channel_repository: C,
        contact_repository: CO,
    ) -> Self {
        MessageService {
            repository: Box::new(repository),
            channel_repository: Box::new(channel_repository),
            contact_repository: Box::new(contact_repository),
        }
    }

    pub async fn send_message(&mut self, cmd: &commands::SendMessage) -> Result<(), MessageError> {
        let contact_from = self.get_contact(&cmd.from).await?;
        let contact_to = self.get_contact(&cmd.to).await?;
        let channel = match &cmd.channel_id {
            None => {
                self.create_private_channel(&vec![contact_from.id.clone(), contact_to.id.clone()]).await?
            }
            Some(c) => self.get_channel(c).await?,
        };
        let message = Message::new(&channel.id, &cmd.from, &cmd.to, &cmd.content);
        match self.repository.create(&message).await {
            Ok(_) => Ok(()),
            Err(e) => Err(MessageError { message: e }),
        }
    }

    async fn get_contact(&mut self, id: &String) -> Result<Contact, MessageError> {
        match self.contact_repository.get(id).await {
            None => Err(MessageError {
                message: format!("Contact with id {} not found", id),
            }),
            Some(c) => Ok(c),
        }
    }

    async fn get_channel(&mut self, id: &String) -> Result<Channel, MessageError> {
        match self.channel_repository.get(id).await {
            None => Err(MessageError {
                message: format!("Channel with id {} not found", id),
            }),
            Some(c) => Ok(c),
        }
    }

    async fn create_private_channel(
        &mut self,
        contact_ids: &Vec<String>,
    ) -> Result<Channel, MessageError> {
        match self.channel_repository.get_by_contact_ids(contact_ids) {
            Some(c) => Ok(c),
            None => {
                let channel = Channel::new("", ChannelType::Private, contact_ids);
                match self.channel_repository.create(&channel).await {
                    Ok(c) => Ok(c),
                    Err(e) => Err(MessageError { message: e }),
                }
            }
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
mod tests {
    use super::*;
    use crate::adapters::{mock_channel_repo, mock_contact_repo, mock_repo, Model};
    use crate::models::{ChannelType, Contact};

    async fn add_contacts(repo: &mut Box<dyn Repository<Contact>>) -> Vec<Contact> {
        repo.create(&Contact::new("Jon Snow", "jon@winterfell.com"))
            .await
            .unwrap();
        repo.create(&Contact::new("Arya Stark", "arya@winterfell.com"))
            .await
            .unwrap();
        repo.list().await.unwrap().clone()
    }

    async fn add_channel(repo: &mut Box<dyn ChannelRepository>, contacts: &Vec<Contact>) -> Channel {
        let cmd = commands::CreateChannel {
            name: "The North Remembers".to_string(),
            channel_type: ChannelType::Private,
            contact_ids: vec![contacts[0].id.clone(), contacts[1].id.clone()],
        };
        repo.create(&Channel::new(
            &cmd.name,
            cmd.channel_type.clone(),
            &cmd.contact_ids,
        )).await
        .unwrap()
    }

    #[actix_web::test]
    async fn can_send_message() {
        let mut service =
            MessageService::new(mock_repo(), mock_channel_repo(), mock_contact_repo());
        let contacts = add_contacts(&mut service.contact_repository).await;
        let channel = add_channel(&mut service.channel_repository, &contacts).await;

        let cmd = commands::SendMessage {
            channel_id: None,
            from: contacts[0].id.clone(),
            to: contacts[1].id.clone(),
            content: "The north remembers!".to_string(),
        };
        let res = service.send_message(&cmd).await;

        let channels = service.channel_repository.list().await.unwrap();
        assert_eq!(channels.len(), 1, "Should not have created a new channel");
        assert!(res.is_ok());

        let messages = service.repository.list().await.unwrap();
        assert_eq!(messages.len(), 1, "Should have created a new message");
        let message = messages.first().unwrap();
        assert_eq!(message.from, cmd.from);
        assert_eq!(message.to, cmd.to);
        assert_eq!(message.content, cmd.content);
    }
}
