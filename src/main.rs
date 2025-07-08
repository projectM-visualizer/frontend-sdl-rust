mod app;
mod dummy_audio;
use std::path::PathBuf;

use crate::app::config::Config;
use clap::Parser;
use confique::Config as ConfiqueConfig;

// User specified configuration options.
//
// Defines CLI, env, and config file parameters.
#[derive(Parser, ConfiqueConfig, Clone, Debug)]
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

impl Cli {
    // Overrides `self`` with values of `other``, if they exist
    pub fn merge(&mut self, other: &Cli) {
        if let Some(config_path) = &other.config_path {
            self.config_path = Some(config_path.clone());
        }
        if let Some(frame_rate) = other.frame_rate {
            self.frame_rate = Some(frame_rate);
        }
        if let Some(preset_path) = &other.preset_path {
            self.preset_path = Some(preset_path.clone());
        }
        if let Some(texture_path) = &other.texture_path {
            self.texture_path = Some(texture_path.clone());
        }
        if let Some(beat_sensitivity) = other.beat_sensitivity {
            self.beat_sensitivity = Some(beat_sensitivity);
        }
        if let Some(preset_duration) = other.preset_duration {
            self.preset_duration = Some(preset_duration);
        }
        if let Some(audio_input) = &other.audio_input {
            self.audio_input = Some(audio_input.clone());
        }
    }
}

fn main() -> Result<(), String> {
    // Load CLI flags and env vars
    let cli_config = Cli::parse();

    // Load file config if a path is specified
    let mut config = match &cli_config.config_path {
        Some(config_path) => Cli::builder()
            .file(config_path)
            .load()
            .map_err(|e| e.to_string())?,
        None => Cli::builder().load().map_err(|e| e.to_string())?,
    };

    // Print the config file path if specified
    if let Some(config_path) = &cli_config.config_path {
        println!("Config loaded from: {}", config_path.display());
    }

    // Apply CLI and env vars (override)
    config.merge(&cli_config);

    let app_config = Config {
        frame_rate: config.frame_rate,
        preset_path: config.preset_path,
        texture_path: config.texture_path,
        beat_sensitivity: config.beat_sensitivity,
        preset_duration: config.preset_duration,
    };

    // Initialize the application
    let mut app = app::App::new(app_config);
    app.init();
    app.main_loop();

    Ok(())
}
