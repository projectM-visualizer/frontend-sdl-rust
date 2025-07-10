use crate::app::App;
use core::fmt;
use std::path::PathBuf;

pub type FrameRate = u32;

const RESOURCE_DIR_DEFAULT: &str = "/usr/local/share/projectM";

/// Configuration for the application
/// Parameters are defined here: https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/parameters.h
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
        write!(
            f,
            "  Preset Duration: {}",
            self.preset_duration
                .map_or("Not specified".to_string(), |d| d.to_string())
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
        }
    }
}

impl App {
    pub fn apply_config(&self, config: &Config) {
        let pm = &self.pm;

        // set frame rate if provided
        if let Some(frame_rate) = config.frame_rate {
            pm.set_fps(frame_rate);
        }

        // load presets if provided
        if let Some(preset_path) = &config.preset_path {
            self.add_preset_path(preset_path);
        }

        // load textures if provided
        if let Some(texture_path) = &config.texture_path {
            let paths = [texture_path.clone().into_os_string().into_string().unwrap()];
            pm.set_texture_search_paths(&paths, 1);
        }

        // set beat sensitivity if provided
        if let Some(beat_sensitivity) = config.beat_sensitivity {
            pm.set_beat_sensitivity(beat_sensitivity);
        }

        // set preset duration if provided
        if let Some(preset_duration) = config.preset_duration {
            pm.set_preset_duration(preset_duration);
        }

        // set preset shuffle mode
        // self.playlist.set_shuffle(true);
    }

    pub fn get_frame_rate(&self) -> FrameRate {
        self.pm.get_fps()
    }
}
