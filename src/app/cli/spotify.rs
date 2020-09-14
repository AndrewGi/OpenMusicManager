use crate::app::services::spotify;
use crate::app::web_client::oauth2::{ClientID, ClientSecret};

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
    let client = spotify::client::Client::new_from_oauth2(
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
