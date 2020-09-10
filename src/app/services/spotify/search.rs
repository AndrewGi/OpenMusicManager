use crate::app::services::error::Error;
use crate::app::services::spotify::album::FullAlbum;
use crate::app::services::spotify::artist::Artist;
use crate::app::services::spotify::client::Client;
use crate::app::services::spotify::episode::SimpleEpisode;
use crate::app::services::spotify::paging::PagingObject;
use crate::app::services::spotify::playlist::FullPlaylist;
use crate::app::services::spotify::show::SimpleShow;
use crate::app::services::spotify::track::FullTrack;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum SearchType {
    Album,
    Artist,
    Playlist,
    Track,
    Show,
    Episode,
}
impl SearchType {
    pub fn as_str(self) -> &'static str {
        match self {
            SearchType::Album => "album",
            SearchType::Artist => "artist",
            SearchType::Playlist => "playlist",
            SearchType::Track => "track",
            SearchType::Show => "show",
            SearchType::Episode => "episode",
        }
    }
    pub fn slice_to_search_string(search_types: &[SearchType]) -> String {
        let mut out = search_types
            .iter()
            .copied()
            .map(SearchType::as_str)
            .fold(String::new(), |s, search_type| s + search_type + ", ");
        //remove trailing comma and space
        out.pop();
        out.pop();
        out
    }
}
fn serialize_search_types<S: serde::Serializer>(
    search_types: &&[SearchType],
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(SearchType::slice_to_search_string(*search_types).as_str())
}
#[derive(Clone, serde::Serialize, Debug)]
pub struct SearchRequest<'a> {
    pub q: String,
    #[serde(rename = "type", serialize_with = "serialize_search_types")]
    pub search_type: &'a [SearchType],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_external: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
}

impl<'a> SearchRequest<'a> {
    pub async fn submit(&self, client: &Client) -> Result<SearchResponse, Error> {
        client
            .get(format!("search/?{}", serde_urlencoded::to_string(self)?.as_str()).as_str())
            .await
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SearchResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artists: Option<PagingObject<Artist>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub albums: Option<PagingObject<FullAlbum>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracks: Option<PagingObject<FullTrack>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playlists: Option<PagingObject<FullPlaylist>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shows: Option<PagingObject<SimpleShow>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub episodes: Option<PagingObject<SimpleEpisode>>,
}
impl SearchResponse {}

#[cfg(test)]
pub mod tests {
    use crate::app::services::error::Error;
    use crate::app::services::spotify::search::{SearchRequest, SearchType};

    #[test]
    pub fn search_request_test() -> Result<(), Error> {
        let t = SearchRequest {
            q: "test".to_string(),
            search_type: &[SearchType::Album, SearchType::Track],
            market: None,
            limit: None,
            offset: None,
            include_external: None,
        };
        assert_eq!(
            r#"{"q":"test","type":"album, track"}"#,
            serde_json::to_string(&t)?
        );
        Ok(())
    }
}
