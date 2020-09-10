use tokio::io::AsyncBufRead;

pub struct Player {
    sink: rodio::Sink,
}
impl Player {
    pub fn default_output_device() -> Option<Player> {
        Some(Player::new(rodio::Sink::new(
            &rodio::default_output_device()?,
        )))
    }
    pub fn new(sink: rodio::Sink) -> Player {
        Player { sink }
    }
}
