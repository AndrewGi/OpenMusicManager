pub mod app;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(app::cli::clap_match().await?)
}
