#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Deserialize, serde::Serialize,
)]

pub struct Image {
    pub height: usize,
    pub url: String,
    pub width: usize,
}
