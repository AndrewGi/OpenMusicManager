use super::album::SimpleAlbum;
use super::artist::Artist;
use super::external_ids::ExternalIDs;
use super::external_urls::ExternalURLs;
use crate::services::error::Error;
use crate::services::spotify::client::Client;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct FullTrack {
    pub album: SimpleAlbum,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: usize,
    pub duration_ms: usize,
    pub explicit: bool,
    pub external_ids: ExternalIDs,
    pub external_urls: ExternalURLs,
    pub href: String,
    pub id: String,
    pub is_local: bool,
    pub name: String,
    pub popularity: usize,
    pub preview_url: Option<String>,
    #[serde(rename = "type")]
    pub track_type: String,
    pub uri: String,
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
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: usize,
    pub duration_ms: usize,
    pub explicit: bool,
    pub external_urls: ExternalURLs,
    pub href: String,
    pub id: String,
    pub is_local: bool,
    pub track_number: usize,
    pub name: String,
    pub preview_url: Option<String>,
    #[serde(rename = "type")]
    pub track_type: String,
    pub uri: String,
}
