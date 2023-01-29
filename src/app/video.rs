use crate::app::App;

impl App<'_> {
    pub fn toggle_fullscreen(&mut self) {
        let is_fullscreen = self.window.fullscreen_state();
        self.window
            .set_fullscreen(match is_fullscreen {
                sdl2::video::FullscreenType::True => sdl2::video::FullscreenType::Off,
                _ => sdl2::video::FullscreenType::True,
            })
            .unwrap();
    }
}
