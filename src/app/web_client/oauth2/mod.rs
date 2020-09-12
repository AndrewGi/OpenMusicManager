#[derive(Clone, Default, Debug)]
pub struct AuthCode(pub String);
#[derive(Clone, Default, Debug)]
pub struct ClientID(pub String);

#[derive(Clone, Default, Debug)]
pub struct ClientSecret(pub String);

#[derive(Clone, Default, Debug)]
pub struct RedirectURI(pub String);
pub mod client;
pub mod token;
pub mod web_server;
