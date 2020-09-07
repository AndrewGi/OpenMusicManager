use crate::services::error::Error;
use crate::services::spotify::client::Client;
use crate::services::spotify::external_urls::ExternalURLs;
use crate::services::spotify::images::Image;
use crate::services::spotify::public_user::PublicUser;
use crate::services::spotify::track::FullTrack;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Playlist {
    pub collaborative: bool,
    pub external_urls: ExternalURLs,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: bool,
    pub snapshot_id: String,
    pub tracks: Vec<PlaylistTrack>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}
impl Playlist {
    pub async fn get_playlist(client: &Client, playlist_id: &str) -> Result<Playlist, Error> {
        Ok(client
            .get(format!("playlists/{}", playlist_id).as_str())
            .await?
            .json()
            .await?)
    }
}
#[derive(serde:: Serialize, serde::Deserialize, Debug)]
pub struct PlaylistTrack {
    pub added_at: String, //Timestamps are formatted like this YYYY-MM-DDTHH:MM:SSZ
    pub added_by: PublicUser,
    pub is_local: bool,
    pub track: FullTrack,
}
