use crate::v10::V10Impl;
use yapspace_protocol::v10::channel::{CreateMessage, GetChannelMessages, GetDmMessages, Message};

impl<'a> V10Impl<'a> {
    pub async fn get_channel_messages(
        &self,
        channel_id: &str,
        params: &GetChannelMessages,
    ) -> Result<Vec<Message>, reqwest::Error> {
        self.0
            .get_query(&["channels", channel_id, "messages"], params)
            .await
    }

    pub async fn get_channel_message(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Message, reqwest::Error> {
        self.0
            .get(&["channels", channel_id, "messages", message_id])
            .await
    }

    pub async fn create_message(
        &self,
        channel_id: &str,
        message: &CreateMessage,
    ) -> Result<(), reqwest::Error> {
        self.0
            .post(&["channels", channel_id, "messages"], message)
            .await
    }

    pub async fn get_dm_messages(
        &self,
        user_id: &str,
        params: &GetDmMessages,
    ) -> Result<Vec<Message>, reqwest::Error> {
        self.0
            .get_query(&["users", user_id, "messages"], params)
            .await
    }

    pub async fn create_dm_message(
        &self,
        user_id: &str,
        message: &CreateMessage,
    ) -> Result<(), reqwest::Error> {
        self.0.post(&["users", user_id, "messages"], message).await
    }
}
