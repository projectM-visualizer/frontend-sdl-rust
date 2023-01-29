use projectm_rs::core::projectm;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::app::App;
use crate::dummy_audio;

impl App {
    pub fn main_loop(&mut self) {
        // events
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        // renderLoop
        'running: loop {
            // check for event
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            // generate random audio
            dummy_audio::generate_random_audio_data(self.pm);

            // render a frame
            projectm::render_frame(self.pm);

            // swap buffers
            self.window.gl_swap_window();
        }
    }
}
