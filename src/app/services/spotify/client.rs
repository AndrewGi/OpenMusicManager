use super::scope;
use crate::app::services::error::Error;
use crate::app::web_client;
use crate::app::web_client::oauth2;
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

    /// Create a Spotify Web API Client. Automatically calls the OAuth2 process to get the correct
    /// tokens. After Logging, spotify will call the `redirect_uri` with the `AuthToken`. `redirect_uri`
    /// is made with the `format!` statement `format!("http://localhost:{}", auth_port)`.
    /// This `redirect_uri` MUST be in your `redirect_uri`'s of your spotify app. Choose a port
    /// according (doesn't need to be port forwarded)
    pub async fn new_from_oauth2(
        client_id: oauth2::ClientID,
        client_secret: oauth2::ClientSecret,
        scopes: impl Iterator<Item = super::scope::Scope>,
        auth_port: u16,
    ) -> Result<Client, Error> {
        let redirect_uri = format!("http://localhost:{}", auth_port);
        let client = oauth2::client::Client::new(
            client_id,
            client_secret,
            oauth2::RedirectURI(redirect_uri),
            scopes
                .map(scope::Scope::as_str)
                .map(str::to_owned)
                .map(std::borrow::Cow::Owned)
                .collect(),
            url::Url::parse(super::BASE_AUTH_URL)?,
            url::Url::parse(super::BASE_TOKEN_URL)?,
        );
        let code = client.get_code(auth_port).await?;
        let token = client.get_token(code).await?;
        Ok(super::client::Client::new(token))
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
