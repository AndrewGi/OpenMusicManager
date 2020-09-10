use super::album::SimpleAlbum;
use super::artist::Artist;
use super::external_ids::ExternalIDs;
use super::external_urls::ExternalURLs;
use crate::app::services::error::Error;
use crate::app::services::spotify::client::Client;
use crate::app::services::spotify::paging::PagingObject;

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
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
        client
            .get(Self::request_url_ending(track_id).as_str())
            .await
    }
    pub async fn get_saved_tracks(client: &Client) -> Result<PagingObject<SavedTrack>, Error> {
        client.get(format!("me/tracks").as_str()).await
    }
    pub async fn get_tracks(client: &Client) -> Result<TrackList, Error> {
        client.get(format!("tracks").as_str()).await
    }
    pub async fn get_audio_analysis(&self, client: &Client) -> Result<AudioAnalysis, Error> {
        client
            .get(format!("audio-analysis/{}", self.id.as_str()).as_str())
            .await
    }
    pub async fn get_audio_features(
        &self,
        //get single track's audio features
        client: &Client,
    ) -> Result<AudioFeatures, Error> {
        client
            .get(format!("audio-features/{}", self.id.as_str()).as_str())
            .await
    }
    /*
    pub async fn chunk_get_audio_features(client: &Client) -> Result<AudioFeaturesList, Error> {}
    */
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

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct SavedTrack {
    pub added_at: String,
    pub track: FullTrack,
}
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct TrackList {
    pub tracks: Vec<FullTrack>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct AudioAnalysis {
    bars: Vec<TimeInterval>,
    beats: Vec<TimeInterval>,
    sections: Vec<Section>,
    segments: Vec<Segment>,
    tatums: Vec<TimeInterval>, //(sic)
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct TimeInterval {
    start: f64,
    duration: f64,
    confidence: f64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct Section {
    start: f64,
    duration: f64,
    confidence: f64,
    loudness: f64,
    tempo: f64,
    tempo_confidence: f64,
    key: usize,
    key_confidence: f64,
    mode: usize,
    mode_confidence: f64,
    time_signature: usize,
    time_signature_confidence: f64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct Segment {
    start: f64,
    duration: f64,
    confidence: f64,
    loudness_start: f64,
    loudness_max: f64,
    loudness_max_time: f64,
    loudness_end: f64,
    pitches: Vec<f64>,
    timbre: Vec<f64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct AudioFeatures {
    duration_ms: usize,
    key: usize,
    mode: usize,
    time_signature: usize,
    acousticness: f64, //(sic)
    danceability: f64, //(sic)
    energy: f64,
    instrumentalness: f64, //(sic)
    liveness: f64,         //(sic)
    loudness: f64,
    speechiness: f64, //(sic)
    valence: f64,
    tempo: f64,
    id: String,
    uri: String,
    track_href: String,
    analysis_url: String,
    #[serde(rename = "type")]
    object_type: String,
}
