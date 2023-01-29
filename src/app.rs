use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use projectm_rs::core::{projectm, projectm_handle};

use crate::dummy_audio;

pub struct App {
    pm: projectm_handle,
    sdl_context: sdl2::Sdl,
    gl_context: sdl2::video::GLContext,
    window: sdl2::video::Window,
}

impl App {
    pub fn new() -> Self {
        // setup sdl
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // request GL version
        // TODO: deal with OpenGL ES here
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_flags().debug().set();
        assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        assert_eq!(gl_attr.context_version(), (3, 3));

        // create window
        // get screen dimensions
        let display_index = 0;
        let display_mode = video_subsystem.desktop_display_mode(display_index).unwrap();
        let window_width = display_mode.w as u32;
        let window_height = display_mode.h as u32;
        let window = video_subsystem
            .window("frontend-sdl2-rust", window_width, window_height)
            .opengl()
            .position_centered()
            .allow_highdpi()
            .build()
            .expect("could not initialize video subsystem");

        // create openGL context
        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();

        // initialize projectM
        let pm = projectm::create();

        // get/set window size
        let (width, height) = window.drawable_size(); // highDPI aware
        projectm::set_window_size(pm, width.try_into().unwrap(), height.try_into().unwrap());

        println!("projectm initialized!");
        Self {
            pm,
            sdl_context,
            gl_context,
            window,
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

            // render a frame
            projectm::render_frame(self.pm);

            // swap buffers
            self.window.gl_swap_window();
        }
    }
}
