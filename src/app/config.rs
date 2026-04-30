use crate::app::App;
use core::fmt;
use std::path::PathBuf;

pub type FrameRate = u32;

const RESOURCE_DIR_DEFAULT: &str = "/usr/local/share/projectM";

/// Configuration for the application
/// Parameters are defined here: https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/parameters.h
#[derive(Clone)]
pub struct Config {
    /// Frame rate to render at. Defaults to 60.
    pub frame_rate: Option<FrameRate>,

    /// Path to the preset directory. Defaults to /usr/local/share/projectM/presets
    pub preset_path: Option<PathBuf>,

    /// Path to the texture directory. Defaults to /usr/local/share/projectM/textures
    pub texture_path: Option<PathBuf>,

    /// How sensitive the beat detection is. 1.0 is default.
    pub beat_sensitivity: Option<f32>,

    /// How long to play a preset before switching to a new one (seconds).
    pub preset_duration: Option<f64>,

    /// Whether to shuffle presets.
    pub shuffle_enabled: Option<bool>,

    /// Soft-cut (crossfade blend) duration between presets (seconds).
    /// Maps to projectM.transitionDuration in .properties files.
    pub soft_cut_duration: Option<f64>,

    /// Whether the current preset is locked (won't auto-switch).
    pub preset_locked: Option<bool>,

    /// Audio device name to use for capture.
    pub audio_device: Option<String>,

    /// Window width.
    pub window_width: Option<u32>,

    /// Window height.
    pub window_height: Option<u32>,

    /// Window X position.
    pub window_left: Option<i32>,

    /// Window Y position.
    pub window_top: Option<i32>,

    /// Monitor index.
    pub window_monitor: Option<u32>,

    /// Whether to override default window position.
    pub window_override_position: Option<bool>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "  Preset path: {}",
            self.preset_path.as_ref().map_or("None".to_string(), |p| p
                .canonicalize()
                .unwrap_or_else(|_| p.clone())
                .display()
                .to_string())
        )?;
        writeln!(
            f,
            "  Texture path: {}",
            self.texture_path.as_ref().map_or("None".to_string(), |p| p
                .canonicalize()
                .unwrap_or_else(|_| p.clone())
                .display()
                .to_string())
        )?;
        writeln!(
            f,
            "  Frame Rate: {}",
            self.frame_rate
                .map_or("Not specified".to_string(), |r| r.to_string())
        )?;
        writeln!(
            f,
            "  Beat Sensitivity: {}",
            self.beat_sensitivity
                .map_or("Not specified".to_string(), |s| s.to_string())
        )?;
        writeln!(
            f,
            "  Preset Duration: {}",
            self.preset_duration
                .map_or("Not specified".to_string(), |d| d.to_string())
        )?;
        writeln!(
            f,
            "  Shuffle: {}",
            self.shuffle_enabled
                .map_or("Not specified".to_string(), |s| s.to_string())
        )?;
        writeln!(
            f,
            "  Soft-cut Duration: {}",
            self.soft_cut_duration
                .map_or("Not specified".to_string(), |d| d.to_string())
        )?;
        write!(
            f,
            "  Audio Device: {}",
            self.audio_device
                .as_ref()
                .map_or("default".to_string(), |d| d.clone())
        )
    }
}

#[cfg(target_os = "macos")]
fn default_resource_dir() -> std::path::PathBuf {
    // current_exe() => /Path/To/projectm_sdl.app/Contents/MacOS/projectm_sdl
    let exe_path = std::env::current_exe().unwrap();
    // Jump up two levels to Contents/, then into Resources
    exe_path
        .parent() // MacOS
        .unwrap()
        .parent() // Contents
        .unwrap()
        .join("Resources")
}

#[cfg(not(target_os = "macos"))]
fn default_resource_dir() -> std::path::PathBuf {
    // On Linux, Windows, etc., do as you wish
    "/usr/local/share/projectM".into()
}

impl Default for Config {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        let resource_dir = default_resource_dir(); // points to .app/Contents/Resources

        #[cfg(not(target_os = "macos"))]
        let resource_dir = std::path::PathBuf::from("/usr/local/share/projectM");

        // Construct paths
        let presets_path = resource_dir.join("presets");
        let textures_path = resource_dir.join("textures");

        Self {
            preset_path: presets_path.exists().then(|| presets_path),
            texture_path: textures_path.exists().then(|| textures_path),
            frame_rate: Some(60),
            beat_sensitivity: Some(1.0),
            preset_duration: Some(10.0),
            shuffle_enabled: None,
            soft_cut_duration: None,
            preset_locked: None,
            audio_device: None,
            window_width: None,
            window_height: None,
            window_left: None,
            window_top: None,
            window_monitor: None,
            window_override_position: None,
        }
    }
}

impl App {
    pub fn apply_config(&mut self, config: &Config) {
        let pm = &self.pm;

        // set frame rate if provided
        if let Some(frame_rate) = config.frame_rate {
            pm.set_fps(frame_rate);
        }

        // set beat sensitivity if provided
        if let Some(beat_sensitivity) = config.beat_sensitivity {
            pm.set_beat_sensitivity(beat_sensitivity);
        }

        // set preset duration if provided (must be set before play_next so the
        // correct duration is in effect when the first preset starts its timer)
        if let Some(preset_duration) = config.preset_duration {
            pm.set_preset_duration(preset_duration);
        }

        // set soft-cut (crossfade) duration if provided
        // maps to projectM.transitionDuration in .properties files
        if let Some(soft_cut_duration) = config.soft_cut_duration {
            pm.set_soft_cut_duration(soft_cut_duration);
        }

        // set shuffle mode
        if let Some(shuffle) = config.shuffle_enabled {
            self.playlist.set_shuffle(shuffle);
        }

        // load presets and start playback (after duration is configured)
        if let Some(preset_path) = &config.preset_path {
            self.add_preset_path(preset_path);
            // Trigger playback of the first preset from the loaded path
            self.playlist.play_next();
        }

        // load textures if provided
        if let Some(texture_path) = &config.texture_path {
            let paths = [texture_path.clone().into_os_string().into_string().unwrap()];
            pm.set_texture_search_paths(&paths, 1);
        }

        // Note: preset_locked from .properties is acknowledged but the projectM
        // playlist API doesn't expose a direct lock method in the Rust bindings.
        // Users can press the preset lock key at runtime instead.
    }

    pub fn get_frame_rate(&self) -> FrameRate {
        self.pm.get_fps()
    }
}
