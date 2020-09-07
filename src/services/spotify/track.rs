use super::album::SimpleAlbum;
use super::artist::Artist;
use super::external_ids::ExternalIDs;
use super::external_urls::ExternalURLs;
use super::track_link::TrackLink;
use crate::services::error::Error;
use crate::services::spotify::client::Client;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct FullTrack {
    album: SimpleAlbum,
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    disc_number: usize,
    duration_ms: usize,
    explicit: bool,
    external_ids: ExternalIDs,
    external_urls: ExternalURLs,
    href: String,
    id: String,
    is_local: bool,
    name: String,
    popularity: usize,
    preview_url: Option<String>,
    #[serde(rename = "type")]
    track_type: String,
    uri: String,
}
impl FullTrack {
    pub fn request_url_ending(track_id: &str) -> String {
        format!("tracks/{}", track_id)
    }
    pub async fn get_track(client: &Client, track_id: &str) -> Result<FullTrack, Error> {
        Ok(client
            .get(Self::request_url_ending(track_id).as_str())
            .await?
            .json()
            .await?)
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct SimpleTrack {
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    disc_number: usize,
    duration_ms: usize,
    explicit: bool,
    external_urls: ExternalURLs,
    href: String,
    id: String,
    is_local: bool,
    track_number: usize,
    name: String,
    preview_url: Option<String>,
    #[serde(rename = "type")]
    track_type: String,
    uri: String,
}
