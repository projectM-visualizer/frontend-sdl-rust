mod app;
mod dummy_audio;
use std::{path::PathBuf};

use crate::app::config::Config;
use clap::Parser;
use confique::Config as ConfiqueConfig;

// command line arguments
#[derive(Parser)]
/// ProjectM: the milkdrop-compatible music visualizer.
///
/// Need help? Join discord: https://discord.gg/uSSggaMBrv
struct Cli {
    // /// Get help
    // #[clap(short, long)]
    // help: bool,
    /// Path to preset directory
    #[clap(short, long)]
    preset_path: Option<PathBuf>,

    /// Path to texture directory
    #[clap(short, long)]
    texture_path: Option<PathBuf>,

    /// Path to config file
    #[clap(short, long)]
    config_path: Option<PathBuf>,

    /// Audio input device (name or index)
    #[clap(short, long)]
    audio_input: Option<String>,
}

fn main() -> Result<(), String> {
    // parse command line arguments
    let cli = Cli::parse();

    // show help
    // cli.help.then(|| usage());

    // Initialize AppConfig with default values
    let mut app_config = Config::default();

    // Load from environment and config files using confique
    let file_config = match Config::builder()
        .env()
        .file(
            cli.config_path
                .as_ref()
                .unwrap_or(&PathBuf::from("conf.toml")),
        )
        .file("conf.yaml")
        .file("conf.json")
        // .file("/usr/local/share/projectM/config.inp")  TODO: custom parser
        .load()
    {
        Ok(config) => config,
        Err(e) => {
            panic!("Error loading config: {}", e);
        }
    };

    // Merge the loaded configuration into the default one
    app_config.merge(file_config);

    // Override with CLI arguments
    if let Some(preset_path) = cli.preset_path {
        app_config.preset_path = Some(preset_path);
    }
    if let Some(texture_path) = cli.texture_path {
        app_config.texture_path = Some(texture_path);
    }
    // Add other CLI overrides as needed

    // Initialize the application
    let mut app = app::App::new(Some(app_config));
    app.init();
    app.main_loop();



    Ok(())
}
