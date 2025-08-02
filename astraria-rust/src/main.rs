use anyhow::Result;
use astraria_rust::AstrariaApp;

fn main() -> Result<()> {
    env_logger::init();

    log::info!("Starting Astraria Rust port...");

    let app = AstrariaApp::new()?;
    app.run()
}
