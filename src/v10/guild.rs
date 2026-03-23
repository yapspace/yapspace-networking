use crate::v10::V10Impl;
use yapspace_protocol::v10::{guild::Guild, user::User, channel::Channel};

impl<'a> V10Impl<'a> {
    pub async fn get_guild(&self, guild_id: &str) -> Result<Guild, reqwest::Error> {
        self.0.get(&["guilds", guild_id]).await
    }

    pub async fn get_guild_channels(&self, guild_id: &str) -> Result<Vec<Channel>, reqwest::Error> {
        self.0.get(&["guilds", guild_id, "channels"]).await
    }

    pub async fn get_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
    ) -> Result<User, reqwest::Error> {
        self.0.get(&["guilds", guild_id, "members", user_id]).await
    }

    pub async fn list_guild_members(&self, guild_id: &str) -> Result<Vec<User>, reqwest::Error> {
        self.0.get(&["guilds", guild_id, "members"]).await
    }
}
