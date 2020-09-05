use super::album::SimpleAlbum;
use super::artist::SimpleArtist;
use super::external_ids::ExternalIDs;
use super::external_urls::ExternalURLs;
use super::track_link::TrackLink;
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FullTrack {
	album: SimpleAlbum,
	artists: Vec<SimpleArtist>,
	available_markets: Vec<String>,
	disc_number: usize,
	duration_ms: usize,
	explicit: bool,
	external_ids: ExternalIDs,
	external_urls: ExternalURLs,
	href: String,
	id: String,
	is_playable: bool,
	is_local: bool,
	linked_from: TrackLink,
	name: String,
	popularity: usize,
	preview_url: String,
	#[serde(alias = "type")]
	track_type: String,
	uri: String,
}