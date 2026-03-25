use crate::v10::V10Impl;
use yapspace_protocol::v10::channel::Channel;

impl<'a> V10Impl<'a> {
    pub async fn get_channel(&self, channel_id: &str) -> Result<Channel, reqwest::Error> {
        self.0.get(&["channels", channel_id]).await
    }
}
