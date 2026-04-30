mod app;
mod dummy_audio;
mod properties;
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
    /// Path to a config file (supports .toml, .json, .yaml, .properties)
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
    #[arg(env = "PM_BEAT_SENSITIVITY")]
    /// Sensitivity of the beat detection
    beat_sensitivity: Option<f32>,

    #[arg(short = 'd', long)]
    #[arg(env = "PM_PRESET_DURATION")]
    /// Duration (seconds) each preset will play
    preset_duration: Option<f64>,

    #[arg(short, long)]
    #[arg(env = "PM_AUDIO_INPUT")]
    /// Audio input device (name or substring match)
    audio_device: Option<String>,

    #[arg(long)]
    #[arg(env = "PM_SHUFFLE")]
    /// Enable preset shuffling
    shuffle_enabled: Option<bool>,

    #[arg(long)]
    #[arg(env = "PM_TRANSITION_DURATION")]
    /// Duration (seconds) of transitions between presets
    transition_duration: Option<f64>,
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
            audio_device: None,
            shuffle_enabled: None,
            transition_duration: None,
        }
    }
}

impl Settings {
    // Overrides `self` with values of `other`, if they exist
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
        if let Some(audio_device) = &other.audio_device {
            self.audio_device = Some(audio_device.clone());
        }
        if let Some(shuffle_enabled) = other.shuffle_enabled {
            self.shuffle_enabled = Some(shuffle_enabled);
        }
        if let Some(transition_duration) = other.transition_duration {
            self.transition_duration = Some(transition_duration);
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

        let extension = path.extension().and_then(|ext| ext.to_str());

        match extension {
            // .properties format — use custom parser
            Some("properties") => {
                println!("Loading .properties config from: {}", path.display());
                return load_properties_settings(&path);
            }
            // TOML/JSON/YAML — use confique
            Some("toml") | Some("json") | Some("yaml") => {}
            _ => {
                return Err(format!(
                    "invalid config file extension: {:?}. Supported: .toml, .json, .yaml, .properties",
                    path.extension()
                ))
            }
        }

        println!("Loading config from: {}", path.display());

        // Load setting from file
        let settings = Settings::builder()
            .file(path)
            .load()
            .map_err(|e| e.to_string())?;

        return Ok(settings);
    }

    // No path, return empty settings
    return Ok(Settings::default());
}

/// Load settings from a projectMSDL .properties file.
fn load_properties_settings(path: &std::path::Path) -> Result<Settings, String> {
    let props = properties::parse_properties_file(path)?;
    let ps = properties::apply_properties(&props);

    Ok(Settings {
        config_path: Some(path.to_path_buf()),
        frame_rate: None, // Not in .properties format
        preset_path: ps.preset_path,
        texture_path: None, // Not in .properties format
        beat_sensitivity: None, // Not in .properties format
        // displayDuration → preset_duration (how long a preset plays before switching)
        preset_duration: ps.display_duration,
        audio_device: ps.audio_device,
        shuffle_enabled: ps.shuffle_enabled,
        // transitionDuration → soft_cut_duration (crossfade blend time)
        transition_duration: ps.transition_duration,
    })
}

fn load_settings() -> Result<Settings, String> {
    // Load CLI flags and env vars
    let cli = Settings::parse();

    // Load file
    let mut settings = load_settings_file(cli.config_path.clone())?;

    // Override file with CLI/env vars/defaults
    settings.apply(&cli);

    return Ok(settings);
}

fn main() -> Result<(), String> {
    let settings = load_settings()?;

    // Build window config from properties if loaded
    let props_window = if let Some(ref config_path) = settings.config_path {
        if config_path.extension().and_then(|e| e.to_str()) == Some("properties") {
            let props = properties::parse_properties_file(config_path).ok();
            props.map(|p| properties::apply_properties(&p))
        } else {
            None
        }
    } else {
        None
    };

    let app_config = Config {
        frame_rate: settings.frame_rate,
        preset_path: settings.preset_path,
        texture_path: settings.texture_path,
        beat_sensitivity: settings.beat_sensitivity,
        preset_duration: settings.preset_duration,
        shuffle_enabled: settings.shuffle_enabled,
        soft_cut_duration: settings.transition_duration,
        preset_locked: props_window.as_ref().and_then(|p| p.preset_locked),
        audio_device: settings.audio_device,
        window_width: props_window.as_ref().and_then(|p| p.window_width),
        window_height: props_window.as_ref().and_then(|p| p.window_height),
        window_left: props_window.as_ref().and_then(|p| p.window_left),
        window_top: props_window.as_ref().and_then(|p| p.window_top),
        window_monitor: props_window.as_ref().and_then(|p| p.window_monitor),
        window_override_position: props_window.as_ref().and_then(|p| p.window_override_position),
    };

    // Initialize the application
    let mut app = app::App::new(app_config);
    app.init();
    app.main_loop();

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
            .file("test-data/config.toml")
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

    #[test]
    fn test_load_properties() {
        use std::fs;
        use std::io::Write;

        let dir = std::env::temp_dir();
        let path = dir.join("test_load.properties");
        let mut f = fs::File::create(&path).unwrap();
        write!(
            f,
            r#"audio.device: BlackHole 2ch
projectM.displayDuration: 60.549
projectM.shuffleEnabled: true
projectM.transitionDuration: 10
projectM.presetPath: /Users/delta/presets
window.width: 850
window.height: 448
"#
        )
        .unwrap();

        let settings =
            crate::load_settings_file(Some(path.clone())).expect("Properties should load");

        assert_eq!(
            settings.preset_path.as_ref().map(|p| p.to_str().unwrap()),
            Some("/Users/delta/presets")
        );
        assert_eq!(settings.shuffle_enabled, Some(true));
        assert_eq!(settings.transition_duration, Some(10.0));
        assert_eq!(settings.preset_duration, Some(60.549));
        assert_eq!(settings.audio_device.as_deref(), Some("BlackHole 2ch"));

        fs::remove_file(&path).ok();
    }
}
