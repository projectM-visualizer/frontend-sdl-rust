use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use projectm_rs::core::{projectm, projectm_handle};

use crate::dummy_audio;

pub struct App {
    pm: projectm_handle,
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl App {
    pub fn new() -> Self {
        let pm = projectm::create();

        // setup sdl
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        // let audio_subsystem = sdl_context.audio()?;

        // create window
        // get screen dimensions
        let mut display_index = 0;
        let display_mode = video_subsystem.desktop_display_mode(display_index).unwrap();
        let mut window_width = display_mode.w as u32;
        let mut window_height = display_mode.h as u32;
        let window = video_subsystem
            .window("frontend-sdl2-rust", window_width, window_height)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");

        // create canvas/renderer
        let canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");

        // projectm::init
        let projectm_handle = projectm::create();

        projectm::set_window_size(
            projectm_handle,
            canvas.output_size().unwrap().0.try_into().unwrap(),
            canvas.output_size().unwrap().1.try_into().unwrap(),
        );
        println!("projectm initialized!");
        Self {
            pm,
            sdl_context,
            canvas,
        }
    }

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

            // projectm::render
            unsafe {
                projectm::render_frame(self.pm);
            }

            // present/render
            self.canvas.present();
        }
    }
}
