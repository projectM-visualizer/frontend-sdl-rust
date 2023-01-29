use crate::app::App;

impl App {
    /// Add presets to the playlist recursively skipping duplicates.
    pub fn add_preset_path(&self, preset_path: &str) {
        self.playlist.add_path(preset_path, true);
        println!("added preset path: {}", preset_path);
        println!("playlist size: {}", self.playlist.len());
    }

    pub fn playlist_play_next(&mut self) {
        self.playlist.play_next();
    }
    pub fn playlist_play_prev(&mut self) {
        self.playlist.play_prev();
    }
    pub fn playlist_play_random(&mut self) {
        self.playlist.play_random();
    }
}
