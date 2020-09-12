use crate::app::gui::table::{Table, TableState};
use crate::app::track::TrackInfo;
use futures::stream::BoxStream;
use futures::SinkExt;
use futures_channel::mpsc;

pub const APP_TITLE: &'static str = "OpenMusicManager";
#[derive(Clone, Default, Debug)]
pub struct Flags {
    pub services: super::services::Flags,
}
#[derive(Debug)]
pub enum Message {
    NoOp,
    Services(super::services::OutgoingMessage),
}
pub struct Application {
    app_tx: mpsc::Sender<Message>,
    services_tx: mpsc::Sender<super::services::IncomingMessage>,
    app_rx: std::sync::Mutex<Option<ChannelSubscription<Message>>>,
    table: TableState,
    track_infos: Vec<TrackInfo>,
}
impl Application {
    pub const CHANNEL_LEN: usize = 100;
}
impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(mut flags: Flags) -> (Self, iced::Command<Self::Message>) {
        let (app_tx, app_rx) = mpsc::channel(Self::CHANNEL_LEN);
        let (mut services_tx, services_rx) = mpsc::channel(Self::CHANNEL_LEN);
        (
            Application {
                app_tx: app_tx.clone(),
                services_tx: services_tx.clone(),
                app_rx: std::sync::Mutex::new(Some(ChannelSubscription::new(app_rx))),
                table: TableState::new(
                    iced::Length::Shrink,
                    vec![
                        iced::Length::FillPortion(1),
                        iced::Length::FillPortion(1),
                        iced::Length::FillPortion(1),
                    ],
                ),
                track_infos: vec![],
            },
            iced::Command::batch(vec![
                super::services::Services::new(services_rx, app_tx)
                    .run_loop()
                    .into(),
                async move {
                    match (
                        flags.services.spotify.client_id.take(),
                        flags.services.spotify.client_secret.take(),
                    ) {
                        (Some(client_id), Some(client_secret)) => {
                            services_tx
                                .send(
                                    super::services::spotify::IncomingMessage::Authorize {
                                        client_id,
                                        client_secret,
                                    }
                                    .into(),
                                )
                                .await
                                .unwrap();
                            Message::NoOp
                        }

                        _ => Message::NoOp,
                    }
                }
                .into(),
            ]),
        )
    }

    fn title(&self) -> String {
        APP_TITLE.to_owned()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        iced::Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.app_rx
            .try_lock()
            .ok()
            .as_mut()
            .map(std::ops::DerefMut::deref_mut)
            .and_then(Option::take)
            .map(iced::Subscription::from_recipe)
            .unwrap_or(iced::Subscription::none())
    }
    fn view(&mut self) -> iced::Element<'_, Message> {
        Table::with_children(&mut self.table, vec![]).into()
    }
}
// Give each ChannelSubscription a unique `u64` token so we can hash it
static CHANNEL_SUB_HASH: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
#[derive(Debug)]
pub struct ChannelSubscription<T>(pub mpsc::Receiver<T>, u64);
impl<T> ChannelSubscription<T> {
    pub fn new(rx: mpsc::Receiver<T>) -> ChannelSubscription<T> {
        ChannelSubscription(
            rx,
            CHANNEL_SUB_HASH.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        )
    }
}
impl<T: Send + 'static, H: std::hash::Hasher, Event> iced_futures::subscription::Recipe<H, Event>
    for ChannelSubscription<T>
{
    type Output = T;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        self.1.hash(state);
    }

    fn stream(self: Box<Self>, input: BoxStream<Event>) -> BoxStream<Self::Output> {
        use iced_futures::futures::StreamExt;
        // We don't need the input so we free up the memory
        std::mem::drop(input);
        (*self).0.boxed()
    }
}
