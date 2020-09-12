pub mod spotify;

use crate::app::services::error::Error;
use futures::{SinkExt, StreamExt};
use futures_channel::mpsc;

#[derive(Clone, Default, Debug)]
pub struct Flags {
    pub spotify: spotify::Flags,
}
#[derive(Debug)]
pub enum OutgoingMessage {
    Spotify(spotify::OutgoingMessage),
    Done,
}
pub enum IncomingMessage {
    Spotify(spotify::IncomingMessage),
}
impl From<spotify::IncomingMessage> for IncomingMessage {
    fn from(msg: spotify::IncomingMessage) -> IncomingMessage {
        IncomingMessage::Spotify(msg)
    }
}
pub struct Services {
    rx: mpsc::Receiver<IncomingMessage>,
    spotify: spotify::Service,
}
pub struct Channel(pub mpsc::Sender<super::application::Message>);
impl Channel {
    pub async fn send(&mut self, msg: OutgoingMessage) -> Result<(), mpsc::SendError> {
        self.0
            .send(super::application::Message::Services(msg))
            .await
    }
}
impl Services {
    pub fn new(
        rx: mpsc::Receiver<IncomingMessage>,
        tx: mpsc::Sender<super::application::Message>,
    ) -> Services {
        Services {
            rx,
            spotify: spotify::Service::new(Channel(tx)),
        }
    }
    pub async fn handle_msg(&mut self, msg: IncomingMessage) -> Result<(), Error> {
        match msg {
            IncomingMessage::Spotify(m) => self.spotify.handle_message(m).await,
        }
    }
    pub async fn run_loop(mut self) -> super::application::Message {
        while let Some(i) = self.rx.next().await {
            if let Err(e) = self.handle_msg(i).await {
                unimplemented!("error handling isn't implemented yet: {:?}", e)
            }
        }
        super::application::Message::Services(OutgoingMessage::Done)
    }
}

pub struct ServicesGui {}
