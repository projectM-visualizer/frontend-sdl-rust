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
    }
}
