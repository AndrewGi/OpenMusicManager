use crate::services::error::Error;
use crate::services::spotify::artist::Artist;
use crate::services::spotify::client::Client;
use crate::services::spotify::copyright::Copyright;
use crate::services::spotify::external_ids::ExternalIDs;
use crate::services::spotify::external_urls::ExternalURLs;
use crate::services::spotify::images::Image;
use crate::services::spotify::track::SimpleTrack;
use serde::Deserializer;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AlbumType {
    Album,
    Single,
    Compilation,
}
impl serde::Serialize for AlbumType {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlbumType {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt::Write;
        const EXPECTED: &'static str = "'album', 'single' or 'compilation'";
        struct AlbumTypeVisitor;
        impl serde::de::Visitor<'_> for AlbumTypeVisitor {
            type Value = AlbumType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str(EXPECTED)
            }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                match value {
                    "album" => Ok(AlbumType::Album),
                    "single" => Ok(AlbumType::Single),
                    "compilation" => Ok(AlbumType::Compilation),
                    _ => Err(E::custom(format!("expected '{}' got {}", EXPECTED, value))),
                }
            }
        }
        deserializer.deserialize_str(AlbumTypeVisitor)
    }
}
impl AlbumType {
    pub fn as_str(self) -> &'static str {
        match self {
            AlbumType::Album => "album",
            AlbumType::Single => "single",
            AlbumType::Compilation => "compilation",
        }
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct SimpleAlbum {
    pub album_group: String,
    pub album_type: AlbumType,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalURLs,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
pub struct FullAlbum {
    pub album_type: AlbumType,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>, //it says copyright objects? might have to make a struct
    pub external_ids: ExternalIDs,
    pub external_urls: ExternalURLs,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub label: String,
    pub name: String,
    pub popularity: usize,
    pub release_date: String,
    pub release_date_precision: String,
    pub tracks: Vec<SimpleTrack>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub uri: String,
}
impl FullAlbum {
    pub async fn get_album(client: &Client, album_id: &str) -> Result<FullAlbum, Error> {
        Ok(client
            .get(format!("albums/{}", album_id).as_str())
            .await?
            .json()
            .await?)
    }
}
