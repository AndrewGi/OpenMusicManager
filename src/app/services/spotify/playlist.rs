use crate::app::services::error::Error;
use crate::app::services::spotify::client::Client;
use crate::app::services::spotify::external_urls::ExternalURLs;
use crate::app::services::spotify::followers::Followers;
use crate::app::services::spotify::images::Image;
use crate::app::services::spotify::paging::PagingObject;
use crate::app::services::spotify::public_user::PublicUser;
use crate::app::services::spotify::track::FullTrack;

#[derive(serde::Serialize)]
struct GetPlaylistsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
}

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct FullPlaylist {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: ExternalURLs,
    pub followers: Followers,
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
impl FullPlaylist {
    pub async fn get_playlist(client: &Client, playlist_id: &str) -> Result<FullPlaylist, Error> {
        client
            .get(format!("playlists/{}", playlist_id).as_str())
            .await
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct SimplePlaylistTracks {
    pub href: String,
    pub total: usize,
}
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct SimplePlaylist {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: ExternalURLs,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: bool,
    pub snapshot_id: String,
    pub tracks: SimplePlaylistTracks,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}
impl SimplePlaylist {
    pub async fn get_full_playlist(&self, client: &Client) -> Result<FullPlaylist, Error> {
        FullPlaylist::get_playlist(client, self.id.as_str()).await
    }
    pub async fn get_current_user_playlists(
        client: &Client,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<PagingObject<SimplePlaylist>, Error> {
        client
            .get(
                format!(
                    "me/playlists?{}",
                    serde_urlencoded::to_string(GetPlaylistsRequest { limit, offset })?.as_str()
                )
                .as_str(),
            )
            .await
    }
    pub async fn get_user_playlists(
        client: &Client,
        user_id: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<PagingObject<SimplePlaylist>, Error> {
        client
            .get(
                format!(
                    "users/{}/playlists?{}",
                    user_id,
                    serde_urlencoded::to_string(GetPlaylistsRequest { limit, offset })?.as_str()
                )
                .as_str(),
            )
            .await
    }
}
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct PlaylistTrack {
    pub added_at: String, //Timestamps are formatted like this YYYY-MM-DDTHH:MM:SSZ
    pub added_by: PublicUser,
    pub is_local: bool,
    pub track: FullTrack,
}
