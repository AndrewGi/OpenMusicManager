use crate::app::gui::table::{ColumnInfo, Table, TableState};
use crate::app::services::error::Error;
use crate::app::services::spotify;
use crate::app::track::TrackInfo;
use crate::app::web_client::oauth2::{ClientID, ClientSecret};

#[derive(Debug)]
pub enum IncomingMessage {
    Authorize {
        client_id: ClientID,
        client_secret: ClientSecret,
    },
}
#[derive(Debug)]
pub enum OutgoingMessage {
    Authorized,
    ShowTracks(Vec<TrackInfo>),
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
                        [].iter().copied(),
                        8888,
                    )
                    .await?,
                );
                self.tx
                    .send(super::OutgoingMessage::Spotify(OutgoingMessage::Authorized))
                    .await?;
                let client = match &mut self.state {
                    State::Ready(c) => c,
                    _ => unreachable!("just set the state above"),
                };
                let res =
                    spotify::album::FullAlbum::get_album(&client, "6QAKD1wqCrmkBYw1AsZfEy").await?;
                let new_tracks = res
                    .tracks
                    .items
                    .iter()
                    .map(|t| TrackInfo {
                        name: t.name.clone(),
                        artist: t.artists[0].name.clone(),
                        album_string: res.name.clone(),
                    })
                    .collect();
                self.tx
                    .send(super::OutgoingMessage::Spotify(
                        OutgoingMessage::ShowTracks(new_tracks),
                    ))
                    .await?;
            }
        }
        Ok(())
    }
}

pub struct ServiceGui {
    table: TableState,
    track_infos: Vec<TrackInfo>,
}
impl ServiceGui {
    pub fn new() -> ServiceGui {
        ServiceGui {
            table: TableState::new(
                iced::Length::Shrink,
                vec![
                    ColumnInfo::new("title".to_owned(), iced::Length::FillPortion(1)),
                    ColumnInfo::new("artist".to_owned(), iced::Length::FillPortion(1)),
                    ColumnInfo::new("album".to_owned(), iced::Length::FillPortion(1)),
                ],
            ),
            track_infos: vec![],
        }
    }
    pub fn update(
        &mut self,
        msg: OutgoingMessage,
    ) -> iced::Command<super::super::application::Message> {
        match msg {
            OutgoingMessage::Authorized => {}
            OutgoingMessage::Error(_) => {}
            OutgoingMessage::ShowTracks(t) => self.track_infos = t,
        }
        iced::Command::none()
    }
    pub fn table(&mut self) -> Table<super::super::application::Message> {
        let rows: Vec<_> = self
            .track_infos
            .iter()
            .map(|i| {
                self.table.table.format_row(
                    [&i.name, &i.artist, &i.album_string]
                        .iter()
                        .map(|s| iced::Text::new(s.clone()).into()),
                )
            })
            .map(|r| r.into())
            .collect();
        let l = rows.len();
        dbg!(l);
        Table::with_children(&mut self.table, rows).into()
    }
    pub fn view(&mut self) -> iced::Element<'_, super::super::application::Message> {
        self.table().into()
    }
}
