use crate::app::services::error::Error;

pub mod gui;
pub mod spotify;
pub fn clap_app() -> clap::App<'static, 'static> {
    clap::App::new("OpenMusicManager")
        .version("0.1")
        .author("Andrew And Null")
        .about("Manage multiple music services")
        .subcommand(spotify::spotify_clap_app())
        .subcommand(gui::gui_clap_app())
}
pub async fn clap_match() -> Result<(), Error> {
    let matches = clap_app().get_matches();
    match matches.subcommand() {
        ("spotify", Some(spotify_matches)) => {
            spotify::spotify_clap_matches(spotify_matches).await?
        }
        ("gui", Some(gui_matches)) => gui::gui_clap_matches(gui_matches).await?,
        (e, _) => eprintln!("unrecognized subcommand '{}'", e),
    }

    Ok(())
}
