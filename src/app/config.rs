use crate::app::App;
use projectm::core::Projectm;
use std::path::Path;

pub type FrameRate = u32;

/// Configuration for the application
/// TODO: use config crate to support loading from env/CLI/file.
/// Parameters are defined here: https://github.com/projectM-visualizer/projectm/blob/master/src/api/include/projectM-4/parameters.h
pub struct Config {
    /// Frame rate to render at. Defaults to 60.
    pub frame_rate: Option<FrameRate>,

    /// Path to the preset directory. Defaults to /usr/local/share/projectM/presets
    pub preset_path: Option<String>,

    /// Path to the texture directory. Defaults to /usr/local/share/projectM/textures
    pub texture_path: Option<String>,

    /// How sensitive the beat detection is. 1.0 is default.
    pub beat_sensitivity: Option<f32>,

    /// How long to play a preset before switching to a new one (seconds).
    pub preset_duration: Option<f64>,
}

impl Default for Config {
    fn default() -> Self {
        // get paths to presets and textures
        // TODO: get from config file or env
        //
        // use /usr/local/share/projectM if it exists, otherwise use local paths
        let resource_dir = "/usr/local/share/projectM";
        let dir_exists = Path::new(&resource_dir).exists();
        let preset_path = if dir_exists {
            String::from(resource_dir) + "/presets"
        } else {
            // just test presets
            "./presets".to_owned()
        };
        let texture_path = if dir_exists {
            String::from(resource_dir) + "/textures"
        } else {
            // doesn't exist
            "./textures".to_owned()
        };

        Self {
            preset_path: Path::new(&preset_path).exists().then(|| preset_path),
            texture_path: Path::new(&texture_path).exists().then(|| texture_path),
            frame_rate: Some(60),
            beat_sensitivity: Some(1.0),
            preset_duration: Some(10.0),
        }
    }
}

impl App {
    pub fn load_config(&self, config: &Config) {
        let pm = *self.pm.lock().unwrap();

        // load presets if provided
        if let Some(preset_path) = &config.preset_path {
            self.add_preset_path(preset_path);
        }

        // set frame rate if provided
        if let Some(frame_rate) = config.frame_rate {
            Projectm::set_fps(pm, frame_rate)
        }

        // load textures if provided
        if let Some(texture_path) = &config.texture_path {
            let mut paths: Vec<String> = Vec::new();
            paths.push(texture_path.into());
            Projectm::set_texture_search_paths(pm, &paths, 1);
        }

        // set beat sensitivity if provided
        if let Some(beat_sensitivity) = config.beat_sensitivity {
            Projectm::set_beat_sensitivity(pm, beat_sensitivity);
        }

        // set preset duration if provided
        if let Some(preset_duration) = config.preset_duration {
            Projectm::set_preset_duration(pm, preset_duration);
        }

        // set preset shuffle mode
        // self.playlist.set_shuffle(true);
    }

    pub fn get_frame_rate(&self) -> FrameRate {
        let pm = *self.pm.lock().unwrap();
        Projectm::get_fps(pm)
    }
}
