use crate::app::services::error::Error;
use crate::app::web_client::oauth2::token::TokenResponse;

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    tokens: TokenResponse,
    client: reqwest::Client,
}
impl Client {
    pub fn new(base_url: String, tokens: TokenResponse) -> Client {
        Client {
            base_url,
            tokens,
            client: reqwest::Client::new(),
        }
    }
    pub fn tokens(&self) -> &TokenResponse {
        &self.tokens
    }
    pub fn url_base(&self) -> &str {
        self.base_url.as_str()
    }
    pub fn url(&self, url_ending: &str) -> Result<reqwest::Url, url::ParseError> {
        reqwest::Url::parse(format!("{}{}", self.base_url.as_str(), url_ending).as_str())
    }
    pub async fn get<T: serde::de::DeserializeOwned>(&self, url_ending: &str) -> Result<T, Error> {
        Self::send_request(self.request(reqwest::Method::GET, url_ending)?).await
    }
    pub async fn post<T: serde::de::DeserializeOwned>(
        &self,
        url_ending: &str,
        body: String,
    ) -> Result<T, Error> {
        Self::send_request(self.request(reqwest::Method::POST, url_ending)?.body(body)).await
    }

    pub async fn put<T: serde::de::DeserializeOwned>(
        &self,
        url_ending: &str,
        body: String,
    ) -> Result<T, Error> {
        Self::send_request(self.request(reqwest::Method::PUT, url_ending)?.body(body)).await
    }
    pub async fn send_request<T: serde::de::DeserializeOwned>(
        request: reqwest::RequestBuilder,
    ) -> Result<T, Error> {
        Ok(request.send().await?.json().await?)
    }
    pub fn request(
        &self,
        method: reqwest::Method,
        url_ending: &str,
    ) -> Result<reqwest::RequestBuilder, Error> {
        Ok(self.client.request(method, self.url(url_ending)?).header(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(
                format!("Bearer {}", self.tokens.access_token.0.as_str()).as_str(),
            )
            .map_err(|e| Error::InvalidHeaderValue(e))?,
        ))
    }
    pub async fn execute_request(&self, req: reqwest::Request) -> Result<reqwest::Response, Error> {
        Ok(self.client.execute(req).await?)
    }
}
