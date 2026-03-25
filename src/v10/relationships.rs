use crate::v10::V10Impl;
use yapspace_protocol::v10::user::{CreateRelationship, Relationship, SendFriendRequest};

impl<'a> V10Impl<'a> {
    pub async fn get_relationships(&self) -> Result<Vec<Relationship>, reqwest::Error> {
        self.0.get(&["users", "@me", "relationships"]).await
    }

    pub async fn send_friend_request(
        &self,
        friend_request: &SendFriendRequest,
    ) -> Result<(), reqwest::Error> {
        self.0
            .post(&["users", "@me", "relationships"], friend_request)
            .await
    }

    pub async fn create_relationship(
        &self,
        user_id: &str,
        create_relationship: &CreateRelationship,
    ) -> Result<(), reqwest::Error> {
        self.0
            .post(
                &["users", "@me", "relationships", user_id],
                create_relationship,
            )
            .await
    }
}
