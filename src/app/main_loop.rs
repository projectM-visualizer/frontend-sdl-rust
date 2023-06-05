use projectm_rs::core::Projectm;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::app::App;

#[cfg(feature = "dummy_audio")]
use crate::dummy_audio;

impl App {
    pub fn main_loop(&mut self) {
        let config = &self.config;
        let frame_rate = config.frame_rate.unwrap();

        // events
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut timer = self.sdl_context.timer().unwrap();

        // renderLoop
        'running: loop {
            // get start time
            let start_time = timer.ticks();

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
                        self.playlist_play_random();
                    }

                    // Toggle fullscreen (F)
                    Event::KeyUp {
                        keycode: Some(Keycode::F),
                        ..
                    } => {
                        self.toggle_fullscreen();
                    }

                    // Next audio capture input device (ctl-I, cmd-I)
                    Event::KeyUp {
                        keycode: Some(Keycode::I),
                        keymod:
                            sdl2::keyboard::Mod::LCTRLMOD
                            | sdl2::keyboard::Mod::RCTRLMOD
                            | sdl2::keyboard::Mod::LGUIMOD
                            | sdl2::keyboard::Mod::RGUIMOD,
                        ..
                    } => {
                        self.audio.open_next_device();
                    }

                    // default
                    _ => {}
                }
            }

            // generate random audio
            #[cfg(feature = "dummy_audio")]
            dummy_audio::generate_random_audio_data(self.pm);

            // render a frame
            {
                let pm = *self.pm.lock().unwrap();
                Projectm::render_frame(pm);
            }

            // swap buffers
            self.window.gl_swap_window();

            if frame_rate > 0 {
                // calculate frame time
                let frame_time = timer.ticks() - start_time;
                if frame_time < 1000 / frame_rate {
                    // sleep the remaining frame time
                    timer.delay(1000 / frame_rate - frame_time);
                }
            }
        }
    }
}
