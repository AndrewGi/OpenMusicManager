use crate::app::gui::application;
use crate::app::services::error::Error;
use crate::app::services::spotify;
use crate::app::web_client::oauth2::{ClientID, ClientSecret};
pub enum IncomingMessage {
    Authorize {
        client_id: ClientID,
        client_secret: ClientSecret,
    },
}
#[derive(Debug)]
pub enum OutgoingMessage {
    Authorized,
    Error(Error),
}
#[derive(Clone, Default, Debug)]
pub struct Flags {
    pub client_id: Option<ClientID>,
    pub client_secret: Option<ClientSecret>,
}
pub struct Service {
    tx: super::Channel,
    state: State,
}
pub enum State {
    Waiting,
    Ready(spotify::client::Client),
}
impl Service {
    pub fn new(tx: super::Channel) -> Service {
        Service {
            tx,
            state: State::Waiting,
        }
    }
    pub async fn handle_message(&mut self, msg: IncomingMessage) -> Result<(), Error> {
        match msg {
            IncomingMessage::Authorize {
                client_id,
                client_secret,
            } => {
                self.state = State::Ready(
                    spotify::client::Client::new_from_oauth2(
                        client_id,
                        client_secret,
                        [].into_iter().copied(),
                        8888,
                    )
                    .await?,
                );
                self.tx
                    .send(super::OutgoingMessage::Spotify(OutgoingMessage::Authorized))
                    .await?;
            }
        }
        Ok(())
    }
}
