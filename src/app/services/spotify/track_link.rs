use super::external_urls::ExternalURLs;

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct TrackLink {
    pub external_urls: ExternalURLs,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub link_type: String,
    pub hate: String,
}
