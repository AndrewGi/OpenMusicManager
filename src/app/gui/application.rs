use crate::app::gui::table::{Table, TableState};
use crate::app::track::TrackInfo;
use iced::futures::stream::BoxStream;

pub const APP_TITLE: &'static str = "OpenMusicManager";
#[derive(Clone, Default, Debug)]
pub struct Flags {}
#[derive(Clone, Debug)]
pub enum Message {
    NewTrack(TrackInfo),
}
pub struct Application {
    table: TableState,
    track_infos: Vec<TrackInfo>,
}
impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Application {
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
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        APP_TITLE.to_owned()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::NewTrack(t) => self.track_infos.push(t),
        }
        iced::Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::none()
    }
    fn view(&mut self) -> iced::Element<'_, Message> {
        Table::with_children(&mut self.table, vec![]).into()
    }
}
// Give each ChannelSubscription a unique `u64` token so we can hash it
static CHANNEL_SUB_HASH: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub struct ChannelSubscription<T>(tokio::sync::mpsc::Receiver<T>, u64);
impl<T> ChannelSubscription<T> {
    pub fn new(rx: tokio::sync::mpsc::Receiver<T>) -> ChannelSubscription<T> {
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
