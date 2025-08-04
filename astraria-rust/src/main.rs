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
    
    let focus_index = if args.len() > 2 {
        args[2].parse::<usize>().unwrap_or(0)
    } else {
        0 // Default to first body (usually the sun)
    };

    log::info!("Using scenario file: {}", scenario_file);
    log::info!("Focusing on body index: {}", focus_index);

    let app = AstrariaApp::new_with_scenario_and_focus(scenario_file, focus_index)?;
    app.run()
}
