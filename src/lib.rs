pub mod v10;

use reqwest::Client as HttpClient;

use crate::v10::V10Impl;

pub struct NetClient {
    pub http_client: HttpClient,
    pub instance_api_url: String,
}

impl NetClient {
    pub(crate) fn new(instance_api_url: String) -> Self {
        let http_client = HttpClient::new();
        Self {
            http_client,
            instance_api_url,
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        args: &[&str],
    ) -> Result<T, reqwest::Error> {
        let mut url = args.join("/");
        url.insert_str(0, &self.instance_api_url);
        self.http_client.get(url).send().await?.json().await
    }
}

pub struct Client {
    pub(crate) net_client: NetClient,
}

impl Client {
    pub fn new(instance_api_url: String) -> Self {
        let net_client = NetClient::new(instance_api_url);
        Self { net_client }
    }
}

impl<'a> Client {
    pub fn v10(&'a self) -> V10Impl<'a> {
        V10Impl(&self.net_client)
    }
}
