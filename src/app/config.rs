use crate::app::App;
use projectm_rs::core::Projectm;

pub type FrameRate = u32;

/// Configuration for the application
/// TODO: use config crate to support loading from env/CLI/file.
pub struct Config {
    pub frame_rate: Option<FrameRate>,
    pub preset_path: Option<String>,
    pub texture_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        // default preset path
        Self {
            // TODO: load from home dir or w/e
            preset_path: Some("/usr/local/share/projectM/presets".to_owned()),
            texture_path: Some("/usr/local/share/projectM/textures".to_owned()),
            frame_rate: Some(60),
        }
    }
}

impl App {
    pub fn load_config(&self, config: &Config) {
        // load presets if provided
        if let Some(preset_path) = &config.preset_path {
            self.add_preset_path(preset_path);
        }

        // set frame rate if provided
        if let Some(frame_rate) = config.frame_rate {
            Projectm::set_fps(self.pm, frame_rate)
        }

        // load textures if provided
        if let Some(texture_path) = &config.texture_path {
            let mut paths: Vec<String> = Vec::new();
            paths.push(texture_path.into());
            Projectm::set_texture_search_paths(self.pm, &paths, 1);
        }
    }

    pub fn get_frame_rate(&self) -> FrameRate {
        Projectm::get_fps(self.pm)
    }
}
