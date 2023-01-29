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
                    // quit (Esc)
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }

                    // Next preset (N, right-arrow)
                    Event::KeyUp {
                        keycode: Some(Keycode::N),
                        ..
                    } => {
                        self.playlist_play_next();
                    }
                    // XXX: how to collapse these into one case?
                    Event::KeyUp {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        self.playlist_play_next();
                    }

                    // Previous preset (P, left-arrow)
                    Event::KeyUp {
                        keycode: Some(Keycode::P),
                        ..
                    } => {
                        self.playlist_play_prev();
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        self.playlist_play_prev();
                    }

                    // Random preset (R)
                    Event::KeyUp {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        self.playlist.play_random();
                    }

                    // Toggle fullscreen (F)
                    Event::KeyUp {
                        keycode: Some(Keycode::F),
                        ..
                    } => {
                        self.toggle_fullscreen();
                    }

                    // default
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
