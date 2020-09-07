use crate::services::error::Error;
use crate::services::spotify::client::Client;
use crate::services::spotify::external_urls::ExternalURLs;
use crate::services::spotify::followers::Followers;
use crate::services::spotify::images::Image;

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
        Ok(client
            .get(format!("artists/{}", artist_id).as_str())
            .await?
            .json()
            .await?)
    }
}
