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
#[command(version)]
/// ProjectM: the milkdrop-compatible music visualizer.
///
/// Need help? Join discord: https://discord.gg/uSSggaMBrv
struct Settings {
    #[arg(short, long = "config")]
    /// Path to a config file
    config_path: Option<PathBuf>,

    #[arg(short, long)]
    #[arg(default_value = "60")]
    #[arg(env = "PM_FRAME_RATE")]
    /// Frame rate to render at.
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
    /// Sensitivity of the beat detection
    beat_sensitivity: Option<f32>,

    #[arg(short = 'd', long)]
    #[arg(default_value = "10")]
    #[arg(env = "PM_PRESET_DURATION")]
    /// Duration (seconds) each preset will play
    preset_duration: Option<f64>,
    // TODO: Add option for specifying audio device
    // #[arg(short, long)]
    // #[arg(env = "PM_AUDIO_INPUT")]
    // /// Audio input device (name or index)
    // audio_input: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            config_path: None,
            frame_rate: None,
            preset_path: None,
            texture_path: None,
            beat_sensitivity: None,
            preset_duration: None,
        }
    }
}

impl Settings {
    // Overrides `self`` with values of `other``, if they exist
    fn apply(&mut self, other: &Settings) {
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
    }
}

fn load_settings_file(path: Option<PathBuf>) -> Result<Settings, String> {
    // Load file config if a path is specified
    if let Some(path) = path {
        // ensure the path exists
        if !path.exists() {
            return Err(format!("config path invalid: {}", path.display()));
        }
        // ensure extention is valid
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") | Some("json") | Some("yaml)") => {}
            _ => {
                return Err(format!(
                    "invalid config file extension: {:?}",
                    path.extension()
                ))
            }
        }

        println!("Loading config from: {}", path.display());

        let settings = Settings::builder()
            .file(path)
            .load()
            .map_err(|e| e.to_string())?;

        return Ok(settings);
    }

    return Ok(Settings {
        config_path: None,
        frame_rate: None,
        preset_path: None,
        texture_path: None,
        beat_sensitivity: None,
        preset_duration: None,
    });
}

fn load_settings() -> Result<Settings, String> {
    // Load CLI flags and env vars
    let cli = Settings::parse();

    let mut settings = load_settings_file(cli.config_path.clone())?;

    // Apply CLI and env vars (override)
    settings.apply(&cli);

    return Ok(settings);
}

fn main() -> Result<(), String> {
    let settings = load_settings()?;

    let app_config = Config {
        frame_rate: settings.frame_rate,
        preset_path: settings.preset_path,
        texture_path: settings.texture_path,
        beat_sensitivity: settings.beat_sensitivity,
        preset_duration: settings.preset_duration,
    };

    // Initialize the application
    let mut app = app::App::new(app_config);
    app.init();
    // app.main_loop();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Settings;
    use clap::Parser;
    use confique::Config;

    fn assert_settings(s: Settings) {
        assert_eq!(s.frame_rate, Some(60));
        assert_eq!(
            s.preset_path.as_ref().map(|p| p.to_str().unwrap()),
            Some("/home/user/.local/share/projectm/presets")
        );
        assert_eq!(
            s.texture_path.as_ref().map(|p| p.to_str().unwrap()),
            Some("/home/user/.local/share/projectm/textures")
        );
        assert_eq!(s.beat_sensitivity, Some(1.0));
        assert_eq!(s.preset_duration, Some(10.0));
    }

    #[test]
    fn test_load_toml() {
        let res = Settings::builder()
            .file("test-data/settings.toml")
            .load()
            .expect("TOML settings should load");

        assert_settings(res);
    }

    #[test]
    fn test_load_env_vars() {
        std::env::set_var("PM_FRAME_RATE", "60");
        std::env::set_var("PM_PRESET_PATH", "/home/user/.local/share/projectm/presets");
        std::env::set_var(
            "PM_TEXTURE_PATH",
            "/home/user/.local/share/projectm/textures",
        );
        std::env::set_var("PM_BEAT_SENSITIVITY", "1.0");
        std::env::set_var("PM_PRESET_DURATION", "10.0");
        std::env::set_var("PM_AUDIO_INPUT", "default");

        // Environment variables are loaded through CLI parsing in clap
        // Create a fake CLI args with no arguments to trigger env var loading
        let args = vec!["test_program"];
        let res = Settings::try_parse_from(args)
            .expect("Environment variable settings should load through CLI parsing");

        assert_settings(res);

        // Clean up environment variables
        std::env::remove_var("PM_FRAME_RATE");
        std::env::remove_var("PM_PRESET_PATH");
        std::env::remove_var("PM_TEXTURE_PATH");
        std::env::remove_var("PM_BEAT_SENSITIVITY");
        std::env::remove_var("PM_PRESET_DURATION");
        std::env::remove_var("PM_AUDIO_INPUT");
    }
}
