use crate::app::App;

impl App {
    pub fn toggle_fullscreen(&mut self) {
        let is_fullscreen = self.window.fullscreen_state();
        self.window
            .set_fullscreen(match is_fullscreen {
                sdl3::video::FullscreenType::True => false,
                _ => true,
            })
            .unwrap();
        self.update_projectm_window_size();
    }

    pub fn update_projectm_window_size(&mut self) {
        let (width, height) = self.window.size();
        self.pm.set_window_size(width as usize, height as usize);
    }
}
