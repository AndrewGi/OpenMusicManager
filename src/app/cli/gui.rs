use crate::app::gui;
use crate::app::web_client::oauth2;
use iced::Application;
pub fn gui_clap_app() -> clap::App<'static, 'static> {
    clap::App::new("gui")
        .help("GUI")
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
pub async fn gui_clap_matches(
    matches: &clap::ArgMatches<'_>,
) -> Result<(), crate::app::services::error::Error> {
    let client_id = matches.value_of("client_id").expect("required by clap"); // We will see "required by clap" if a programmer creates a error (deletes a line or something)
    let client_secret = matches.value_of("client_secret").expect("required by clap");
    let flags = gui::application::Flags {
        services: gui::services::Flags {
            spotify: gui::services::spotify::Flags {
                client_id: Some(oauth2::ClientID(client_id.to_owned())),
                client_secret: Some(oauth2::ClientSecret(client_secret.to_owned())),
            },
        },
    };
    gui::application::Application::run(iced::Settings::with_flags(flags));
    Ok(())
}
