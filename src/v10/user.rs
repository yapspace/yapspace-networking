use crate::v10::V10Impl;
use yapspace_protocol::v10::{guild::PartialGuild, user::User};

impl<'a> V10Impl<'a> {
    pub async fn get_current_user(&self) -> Result<User, reqwest::Error> {
        self.0.get(&["users", "@me"]).await
    }

    pub async fn get_user(&self, id: &str) -> Result<User, reqwest::Error> {
        self.0.get(&["users", id]).await
    }

    pub async fn modify_current_user(
        &self,
        username: Option<String>,
        avatar: Option<()>,
        banner: Option<()>,
    ) -> Result<(), reqwest::Error> {
        todo!()
    }

    pub async fn get_current_user_guilds(&self) -> Result<Vec<PartialGuild>, reqwest::Error> {
        self.0.get(&["users", "@me", "guilds"]).await
    }
}
