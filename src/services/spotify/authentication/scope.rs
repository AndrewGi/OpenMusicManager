pub enum Scope {
    UserReadPlaybackPosition,
    UserReadEmail,
    UserLibraryRead,
    UserTopRead,
    PlaylistModifyPublic,
    UserFollowRead,
    UserReadPlaybackState,
    UserReadCurrentlyPlaying,
    UserReadPrivate,
    PlaylistReadCollaborative,
    PlaylistModifyPrivate,
    UserFollowModify,
    UserModifyPlaybackState,
    UserReadRecentlyPlayed,
}
impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Scope::UserReadPlaybackPosition => "user-read-playback-position",
            Scope::UserReadEmail => "user-read-email",
            Scope::UserLibraryRead => "user-library-read",
            Scope::UserTopRead => "user-top-read",
            Scope::PlaylistModifyPublic => "playlist-modify-public",
            Scope::UserFollowRead => "user-follow-read",
            Scope::UserReadPlaybackState => "user-read-playback-state",
            Scope::UserReadCurrentlyPlaying => "user-read-currently-playing",
            Scope::UserReadPrivate => "user-read-private",
            Scope::PlaylistReadCollaborative => "playlist-read-collaborative",
            Scope::PlaylistModifyPrivate => "playlist-modify-private",
            Scope::UserFollowModify => "user-follow-modify",
            Scope::UserModifyPlaybackState => "user-modify-playback-state",
            Scope::UserReadRecentlyPlayed => "user-read-recently-played",
        }
    }
}
impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
