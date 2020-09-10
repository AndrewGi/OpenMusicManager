use crate::app::services::spotify;
use crate::app::web_client;
use crate::app::web_client::oauth2::{ClientID, ClientSecret, RedirectURI};

pub fn spotify_clap_app() -> clap::App<'static, 'static> {
    clap::App::new("spotify")
        .help("Spotify web API ")
        .arg(
            clap::Arg::with_name("client_id")
                .long("client_id")
                .help("Spotify client ID")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("client_secret")
                .long("client_secret")
                .help("Spotify client secret")
                .required(true)
                .takes_value(true),
        )
}
/// Create a Spotify Web API Client. Automatically calls the OAuth2 process to get the correct
/// tokens. After Logging, spotify will call the `redirect_uri` with the `AuthToken`. `redirect_uri`
/// is made with the `format!` statement `format!("http://localhost:{}", auth_port)`.
/// This `redirect_uri` MUST be in your `redirect_uri`'s of your spotify app. Choose a port
/// according (doesn't need to be port forwarded)
pub async fn spotify_client(
    client_id: ClientID,
    client_secret: ClientSecret,
    scopes: impl Iterator<Item = spotify::scope::Scope>,
    auth_port: u16,
) -> Result<spotify::client::Client, crate::app::services::error::Error> {
    let redirect_uri = format!("http://localhost:{}", auth_port);
    let client = web_client::oauth2::client::Client::new(
        client_id,
        client_secret,
        RedirectURI(redirect_uri),
        scopes
            .map(spotify::scope::Scope::as_str)
            .map(str::to_owned)
            .map(std::borrow::Cow::Owned)
            .collect(),
        url::Url::parse(spotify::BASE_AUTH_URL)?,
        url::Url::parse(spotify::BASE_TOKEN_URL)?,
    );
    let code = client.get_code(auth_port).await?;
    let token = client.get_token(code).await?;
    Ok(spotify::client::Client::new(token))
}
pub async fn spotify_clap_matches(
    matches: &clap::ArgMatches<'_>,
) -> Result<(), crate::app::services::error::Error> {
    let client_id = matches.value_of("client_id").expect("required by clap"); // We will see "required by clap" if a programmer creates a error (deletes a line or something)
    let client_secret = matches.value_of("client_secret").expect("required by clap");
    println!(
        "client_id = {}, client_secret = {}",
        client_id, client_secret
    );
    const AUTH_PORT: u16 = 8888;
    let client = spotify_client(
        ClientID(client_id.to_owned()),
        ClientSecret(client_secret.to_owned()),
        [].iter().copied(),
        AUTH_PORT,
    )
    .await?;
    let res =
        spotify::playlist::SimplePlaylist::get_current_user_playlists(&client, None, None).await?;
    dbg!(&res);
    Ok(())
}
