use crate::app::App;

pub struct Config {
    pub preset_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        // default preset path
        Self {
            // load from home dir or w/e
            preset_path: Some(String::from("/usr/local/share/projectM/presets")),
        }
    }
}

impl App {
    pub fn load_config(&mut self, config: Config) {
        // load presets if provided
        if let Some(preset_path) = config.preset_path {
            self.add_preset_path(&preset_path);
        }
    }
}
