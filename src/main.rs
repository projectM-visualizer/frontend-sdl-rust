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
    #[arg(short, long)]
    /// Path to config file
    config_path: Option<PathBuf>,

    #[arg(short, long)]
    #[arg(default_value = "60")]
    #[arg(env = "PM_FRAME_RATE")]
    frame_rate: Option<u32>,

    #[arg(short, long)]
    #[arg(env = "PM_PRESET_PATH")]
    /// Path to preset directory
    preset_path: Option<PathBuf>,

    #[arg(short, long)]
    #[arg(env = "PM_TEXTURE_PATH")]
    /// Path to texture directory
    texture_path: Option<PathBuf>,

    #[arg(short, long)]
    #[arg(default_value = "1.0")]
    #[arg(env = "PM_BEAT_SENSITIVITY")]
    beat_sensitivity: Option<f32>,

    #[arg(short = 'd', long)]
    #[arg(default_value = "10")]
    #[arg(env = "PM_PRESET_DURATION")]
    preset_duration: Option<f64>,

    #[arg(short, long)]
    #[arg(env = "PM_AUDIO_INPUT")]
    /// Audio input device (name or index)
    audio_input: Option<String>,
}

fn main() -> Result<(), String> {
    let cli_config = Cli::parse();
    let app_config = Config {
        frame_rate: cli_config.frame_rate,
        preset_path: cli_config.preset_path,
        texture_path: cli_config.texture_path,
        beat_sensitivity: cli_config.beat_sensitivity,
        preset_duration: cli_config.preset_duration,
    };

    // Initialize the application
    let mut app = app::App::new(Some(app_config));
    app.init();
    app.main_loop();

    Ok(())
}
