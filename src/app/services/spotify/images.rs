#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Deserialize, serde::Serialize,
)]

pub struct Image {
    pub height: Option<usize>,
    pub url: String,
    pub width: Option<usize>,
}
