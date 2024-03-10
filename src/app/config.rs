use crate::app::App;
use confique::Config as ConfiqueConfig;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub type FrameRate = u32;

const RESOURCE_DIR_DEFAULT: &str = "/usr/local/share/projectM";

/// Configuration for the application
/// Parameters are defined here: https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/parameters.h
// #[derive(Debug, Serialize, Deserialize)]
#[derive(ConfiqueConfig, Debug, Serialize, Deserialize)]
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

impl Default for Config {
    fn default() -> Self {
        let dir_exists = Path::new(&RESOURCE_DIR_DEFAULT).exists();

        let preset_path = format!("{}/presets", RESOURCE_DIR_DEFAULT);
        let texture_path = format!("{}/textures", RESOURCE_DIR_DEFAULT);

        Self {
            preset_path: if dir_exists {
                Some(preset_path.into())
            } else {
                None
            },
            texture_path: if dir_exists {
                Some(texture_path.into())
            } else {
                None
            },
            frame_rate: Some(60),
            beat_sensitivity: Some(1.0),
            preset_duration: Some(10.0),
        }
    }
}

impl Config {
    // Merges another Config into this one
    pub fn merge(&mut self, mut other: Self) {
        self.frame_rate = other.frame_rate.take().or(self.frame_rate.take());
        self.preset_path = other.preset_path.take().or(self.preset_path.take());
        self.texture_path = other.texture_path.take().or(self.texture_path.take());
        self.beat_sensitivity = other
            .beat_sensitivity
            .take()
            .or(self.beat_sensitivity.take());
        self.preset_duration = other.preset_duration.take().or(self.preset_duration.take());
    }
}

impl App {
    pub fn load_config(&self, config: &Config) {
        let pm = &self.pm;

        // load presets if provided
        if let Some(preset_path) = &config.preset_path {
            self.add_preset_path(preset_path);
        }

        // set frame rate if provided
        if let Some(frame_rate) = config.frame_rate {
            pm.set_fps(frame_rate);
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
