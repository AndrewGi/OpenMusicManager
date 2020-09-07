use crate::services::error::Error;
use crate::services::spotify::authentication::token::TokenResponse;

pub struct Client {
    tokens: TokenResponse,
    client: reqwest::Client,
}
impl Client {
    pub const BASE_URL: &'static str = "https://api.spotify.com/v1/";
    pub fn new(tokens: TokenResponse) -> Client {
        Client {
            tokens,
            client: reqwest::Client::new(),
        }
    }
    pub fn tokens(&self) -> &TokenResponse {
        &self.tokens
    }
    pub fn url(url_ending: &str) -> Result<reqwest::Url, url::ParseError> {
        reqwest::Url::parse(format!("{}{}", Self::BASE_URL, url_ending).as_str())
    }
    pub async fn get(&self, url_ending: &str) -> Result<reqwest::Response, Error> {
        Ok(self
            .request(reqwest::Method::GET, url_ending)?
            .send()
            .await?)
    }
    pub fn request(
        &self,
        method: reqwest::Method,
        url_ending: &str,
    ) -> Result<reqwest::RequestBuilder, Error> {
        Ok(self.client.request(method, Self::url(url_ending)?).header(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(
                format!("Bearer {}", self.tokens.access_token.0.as_str()).as_str(),
            )
            .map_err(|e| Error::InvalidHeaderValue(e))?,
        ))
    }
    pub async fn execute_request(
        &self,
        mut req: reqwest::Request,
    ) -> Result<reqwest::Response, Error> {
        Ok(self.client.execute(req).await?)
    }
}
