#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct TrackInfo {
    pub name: String,
    pub artist: String,
    pub album_string: String,
}
