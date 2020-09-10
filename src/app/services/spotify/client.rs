use crate::app::services::error::Error;
use crate::app::web_client;
use crate::app::web_client::oauth2::token::TokenResponse;

pub struct Client {
    web_client: web_client::client::Client,
}
impl Client {
    pub fn new(tokens: TokenResponse) -> Client {
        Client {
            web_client: web_client::client::Client::new(super::BASE_API_URL.to_owned(), tokens),
        }
    }
    pub async fn get<T: serde::de::DeserializeOwned>(&self, url_ending: &str) -> Result<T, Error> {
        self.web_client.get(url_ending).await
    }
    pub async fn post<T: serde::de::DeserializeOwned>(
        &self,
        url_ending: &str,
        body: String,
    ) -> Result<T, Error> {
        self.web_client.post(url_ending, body).await
    }
    pub async fn put<T: serde::de::DeserializeOwned>(
        &self,
        url_ending: &str,
        body: String,
    ) -> Result<T, Error> {
        self.web_client.put(url_ending, body).await
    }
}
