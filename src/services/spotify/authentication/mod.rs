pub struct AuthCode(pub String);
pub struct ClientID(pub String);
pub struct ClientSecret(pub String);
pub struct RedirectURI(pub String);
pub mod client;
pub mod scope;
pub mod token;
pub mod web_server;
