use futures::stream::BoxStream;
use futures::Stream;
use tokio::sync::mpsc;

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
    app_rx: ChannelSubscription<Message>,
    services_gui: super::services::ServicesGui,
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
                app_rx: ChannelSubscription::new(app_rx),
                services_gui: super::services::ServicesGui::new(),
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
        dbg!(&message);
        match message {
            Message::NoOp => iced::Command::none(),
            Message::Services(msg) => self.services_gui.update(msg),
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::from_recipe(self.app_rx.clone())
    }
    fn view(&mut self) -> iced::Element<'_, Message> {
        self.services_gui.view()
    }
}
// Give each ChannelSubscription a unique `u64` token so we can hash it
#[derive(Debug)]
pub struct ChannelSubscription<T>(pub std::sync::Arc<tokio::sync::Mutex<mpsc::Receiver<T>>>); // mpsc
impl<T> ChannelSubscription<T> {
    pub fn new(rx: mpsc::Receiver<T>) -> ChannelSubscription<T> {
        // mpsc
        ChannelSubscription(std::sync::Arc::new(tokio::sync::Mutex::new(rx)))
    }
    pub fn stream(self) -> impl Stream<Item = T> {
        futures::stream::unfold(self, |channel| async move {
            let msg = channel.0.lock().await.recv().await;
            msg.map(move |m| (m, channel))
        })
    }
}
impl<T> Clone for ChannelSubscription<T> {
    fn clone(&self) -> Self {
        ChannelSubscription(self.0.clone())
    }
}
impl<T: 'static + Send, H: std::hash::Hasher, Event> iced_futures::subscription::Recipe<H, Event>
    for ChannelSubscription<T>
{
    type Output = T;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::Any::type_id(self).hash(state);
        std::sync::Arc::as_ptr(&self.0).hash(state);
    }

    fn stream(self: Box<Self>, input: BoxStream<Event>) -> BoxStream<Self::Output> {
        // We don't need the input so we free up the memory
        std::mem::drop(input);

        Box::pin((*self).stream())
    }
}
