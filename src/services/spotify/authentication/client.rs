use crate::services::spotify::authentication::{ClientID, ClientSecret, RedirectURI};

pub struct Client {
    client_id: ClientID,
    client_secret: ClientSecret,
    redirect_uri: RedirectURI,
    scope: String,
}
impl Client {
    pub fn new(
        client_id: ClientID,
        client_secret: ClientSecret,
        redirect_uri: RedirectURI,
        scope: String,
    ) -> Client {
        Client {
            client_id,
            client_secret,
            redirect_uri,
            scope,
        }
    }

    pub fn login_url(&self) -> String {
        const AUTH_URL: &'static str = "https://accounts.spotify.com/authorize";
        let params = &[
            ("client_id", Some(self.client_id.0.as_str())),
            ("scope", Some(self.scope.as_str())),
            ("redirect_uri", Some(self.redirect_uri.0.as_str())),
        ];
        let encoded = serde_urlencoded::to_string(params).expect("params should never fail");
        format!("{}?{}", AUTH_URL, encoded)
    }
}
