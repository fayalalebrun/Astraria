use anyhow::Result;
use astraria_rust::AstrariaApp;
use std::env;

fn main() -> Result<()> {
    env_logger::init();

    log::info!("Starting Astraria Rust port...");

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let scenario_file = if args.len() > 1 {
        args[1].clone()
    } else {
        "Solar_System_2K.txt".to_string()
    };

    log::info!("Using scenario file: {}", scenario_file);

    let app = AstrariaApp::new_with_scenario(scenario_file)?;
    app.run()
}
