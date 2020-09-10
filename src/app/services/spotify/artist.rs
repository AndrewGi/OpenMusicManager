use crate::app::services::error::Error;
use crate::app::services::spotify::album::SimpleAlbum;
use crate::app::services::spotify::client::Client;
use crate::app::services::spotify::external_urls::ExternalURLs;
use crate::app::services::spotify::followers::Followers;
use crate::app::services::spotify::images::Image;
use crate::app::services::spotify::paging::PagingObject;

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct Artist {
    pub external_urls: ExternalURLs,
    pub followers: Followers,
    pub genres: String,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: usize,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}

impl Artist {
    pub async fn get_artist(client: &Client, artist_id: &str) -> Result<Artist, Error> {
        client.get(format!("artists/{}", artist_id).as_str()).await
    }
    pub async fn get_artist_albums(
        client: &Client,
        artist_id: &str,
    ) -> Result<PagingObject<SimpleAlbum>, Error> {
        client
            .get(format!("artists/{}/albums", artist_id).as_str())
            .await
    }
}
