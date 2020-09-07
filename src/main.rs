use crate::services::spotify;
use crate::services::spotify::authentication::scope::Scope;
use crate::services::spotify::authentication::{ClientID, ClientSecret, RedirectURI};
use crate::services::spotify::track::FullTrack;

pub mod services;

pub fn clap_app() -> clap::App<'static, 'static> {
    clap::App::new("OpenMusicManager")
        .version("0.1")
        .author("Andrew And Null")
        .about("Manage multiple music services")
        .subcommand(
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
                ),
        )
}
async fn spotify_clap_matches(
    matches: &clap::ArgMatches<'_>,
) -> Result<(), services::error::Error> {
    let client_id = matches.value_of("client_id").expect("required by clap"); // We will see "required by clap" if a programmer creates a error (deletes a line or something)
    let client_secret = matches.value_of("client_secret").expect("required by clap");
    println!(
        "client_id = {}, client_secret = {}",
        client_id, client_secret
    );
    let auth_port = 8888_u16;
    let redirect_uri = format!("http://localhost:{}", auth_port);
    let client = spotify::authentication::client::Client::new(
        ClientID(client_id.to_owned()),
        ClientSecret(client_secret.to_owned()),
        RedirectURI(redirect_uri),
        &[Scope::PlaylistReadCollaborative],
    );
    let code = client.get_code(auth_port).await?;
    println!("auth code: {}", code.0.as_str());
    let token = client.get_token(code).await?;
    println!("token_res: {:?}", token);

    let client = spotify::client::Client::new(token);
    let res = spotify::search::SearchRequest {
        q: "pink cigarette".to_owned(),
        search_type: &[spotify::search::SearchType::Track],
        market: None,
        limit: None,
        offset: None,
        include_external: None,
    }
    .submit(&client)
    .await?;
    dbg!(&res);

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app().get_matches();
    match matches.subcommand() {
        ("spotify", Some(spotify_matches)) => spotify_clap_matches(spotify_matches).await?,
        (e, _) => eprintln!("unrecognized subcommand '{}'", e),
    }
    Ok(())
}
