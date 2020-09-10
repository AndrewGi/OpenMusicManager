use crate::app::gui;
use iced::Application;
pub fn gui_clap_app() -> clap::App<'static, 'static> {
    clap::App::new("gui").help("GUI")
}
pub async fn gui_clap_matches(
    _matches: &clap::ArgMatches<'_>,
) -> Result<(), crate::app::services::error::Error> {
    gui::application::Application::run(iced::Settings::default());
    Ok(())
}
