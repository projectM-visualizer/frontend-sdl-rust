use crate::app::App;
use projectm_rs::core::Projectm;

pub type FrameRate = u32;

pub struct Config {
    pub frame_rate: Option<FrameRate>,
    pub preset_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        // default preset path
        Self {
            // TODO: load from home dir or w/e
            preset_path: Some(String::from("/usr/local/share/projectM/presets")),
            frame_rate: Some(60),
        }
    }
}

impl App {
    pub fn load_config(&self, config: &Config) {
        // load presets if provided
        if let Some(preset_path) = &config.preset_path {
            self.add_preset_path(&preset_path);
        }

        // set frame rate if provided
        if let Some(frame_rate) = config.frame_rate {
            Projectm::set_fps(self.pm, frame_rate.try_into().unwrap())
        }
    }

    pub fn get_frame_rate(&self) -> FrameRate {
        Projectm::get_fps(self.pm)
    }
}
