use crate::services::error::Error;
use crate::services::spotify::authentication::scope::Scope;
use crate::services::spotify::authentication::token::{AccessToken, RefreshToken, TokenResponse};
use crate::services::spotify::authentication::{AuthCode, ClientID, ClientSecret, RedirectURI};
use std::ops::Add;

pub struct Client<'a> {
    reqwest_client: reqwest::Client,
    client_id: ClientID,
    client_secret: ClientSecret,
    redirect_uri: RedirectURI,
    scopes: &'a [Scope],
}
impl<'a> Client<'a> {
    pub const BASE_URL: &'static str = "https://accounts.spotify.com/api/token";
    pub fn base_url() -> url::Url {
        url::Url::parse(Self::BASE_URL).expect("hard coded url")
    }
    pub fn new(
        client_id: ClientID,
        client_secret: ClientSecret,
        redirect_uri: RedirectURI,
        scopes: &'a [Scope],
    ) -> Self {
        Client {
            reqwest_client: reqwest::Client::new(),
            client_id,
            client_secret,
            redirect_uri,
            scopes,
        }
    }
    pub async fn get_code(&self, web_server_port: u16) -> Result<AuthCode, std::io::Error> {
        webbrowser::open(self.login_url().as_str())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        super::web_server::get_code(web_server_port).await
    }
    async fn post(&self, body: String) -> Result<TokenResponse, Error> {
        let mut res = self
            .reqwest_client
            .post(Self::base_url())
            .body(body)
            .header(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(
                    format!("Basic {}", self.basic_base64_auth().as_str()).as_str(),
                )
                .map_err(|e| Error::InvalidHeaderValue(e))?,
            )
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .send()
            .await?;
        #[derive(serde::Deserialize)]
        pub struct TokenRes {
            access_token: String,
            token_type: String,
            scope: String,
            expires_in: u64,
            refresh_token: String,
        }
        let res_bytes = res.bytes().await?;
        let token_res: TokenRes = serde_json::from_slice(res_bytes.as_ref())?;
        Ok(TokenResponse {
            access_token: AccessToken(token_res.access_token),
            refresh_token: RefreshToken(token_res.refresh_token),
            expires_at: std::time::Instant::now()
                .add(std::time::Duration::from_secs(token_res.expires_in)),
        })
    }
    pub async fn get_new_access_token(
        &self,
        refresh_token: RefreshToken,
    ) -> Result<TokenResponse, Error> {
        #[derive(serde::Serialize)]
        struct TokenRequest<'a> {
            grant_type: &'static str,
            refresh_token: &'a str,
        }
        let post_body = serde_urlencoded::to_string(TokenRequest {
            grant_type: "refresh_token",
            refresh_token: refresh_token.0.as_str(),
        })?;
        self.post(post_body).await
    }
    pub async fn get_token(&self, auth_code: AuthCode) -> Result<TokenResponse, Error> {
        #[derive(serde::Serialize)]
        struct TokenRequest<'a> {
            grant_type: &'static str,
            code: &'a str,
            redirect_uri: &'a str,
        }
        let post_body = serde_urlencoded::to_string(TokenRequest {
            grant_type: "authorization_code",
            code: auth_code.0.as_str(),
            redirect_uri: self.redirect_uri.0.as_str(),
        })?;
        self.post(post_body).await
    }
    pub fn basic_base64_auth(&self) -> String {
        base64::encode(format!("{}:{}", self.client_id.0, self.client_secret.0))
    }
    pub fn login_url(&self) -> String {
        const AUTH_URL: &'static str = "https://accounts.spotify.com/authorize";
        let mut scope_str: String = self
            .scopes
            .iter()
            .map(Scope::as_str)
            .collect::<Vec<_>>()
            .join(" ");
        let params = &[
            ("client_id", Some(self.client_id.0.as_str())),
            ("scope", Some(scope_str.as_str())),
            ("response_type", Some("code")),
            ("redirect_uri", Some(self.redirect_uri.0.as_str())),
        ];
        let encoded = serde_urlencoded::to_string(params).expect("params should never fail");
        format!("{}?{}", AUTH_URL, encoded)
    }
}
