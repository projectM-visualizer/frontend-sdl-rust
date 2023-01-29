use projectm_rs::core::{projectm, projectm_handle};
use sdl2::video::GLProfile;

pub mod audio;
pub mod config;
pub mod main_loop;
pub mod playlist;
pub mod video;

pub struct App {
    pm: projectm_handle,
    playlist: projectm_rs::playlist::Playlist,
    sdl_context: sdl2::Sdl,
    gl_context: sdl2::video::GLContext,
    window: sdl2::video::Window,
    config: config::Config,
    audio: audio::Audio,
}

pub fn default_config() -> config::Config {
    config::Config::default()
}

impl App {
    pub fn new(config: Option<crate::app::config::Config>) -> Self {
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
            .window("ProjectM", window_width, window_height)
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

        // and a preset playlist
        let playlist = projectm_rs::playlist::Playlist::create(pm);

        // get/set window size
        let (width, height) = window.drawable_size(); // highDPI aware
        projectm::set_window_size(pm, width.try_into().unwrap(), height.try_into().unwrap());

        // initialize audio
        let audio = audio::Audio::new(&sdl_context);

        Self {
            pm,
            playlist,
            sdl_context,
            gl_context,
            window,
            config: if let Some(config) = config {
                config
            } else {
                default_config()
            },
            audio,
        }
    }

    pub fn init(&mut self) {
        // load config
        self.load_config(&self.config);
    }
}
