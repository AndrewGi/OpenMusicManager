pub mod services;

pub fn clap_app() -> clap::App<'static, 'static> {
    clap::App::new("OpenMusicManager")
        .version("0.1")
        .author("Andrew And Null")
        .about("Manage multiple music services")
        .arg(
            clap::Arg::with_name("client_id")
                .help("Spotify client ID")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("client_secret")
                .help("Spotify client secret")
                .required(true)
                .takes_value(true),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app().get_matches();
    Ok(())
}
