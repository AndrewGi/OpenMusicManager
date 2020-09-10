#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, serde::Deserialize, serde::Serialize,
)]

pub struct Copyright {
    text: String,
    #[serde(rename = "type")]
    object_type: String,
}
