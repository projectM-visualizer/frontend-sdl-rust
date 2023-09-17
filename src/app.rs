use projectm::core::ProjectM;
use sdl3::video::GLProfile;

pub mod audio;
pub mod config;
pub mod main_loop;
pub mod playlist;
pub mod video;

pub type ProjectMWrapped = ProjectM;

use std::rc::Rc;

/// Application state
pub struct App {
    pm: Rc<ProjectMWrapped>,
    playlist: projectm::playlist::Playlist,
    sdl_context: sdl3::Sdl,
    window: sdl3::video::Window,
    config: config::Config,
    audio: audio::Audio,
    _gl_context: sdl3::video::GLContext,
}

pub fn default_config() -> config::Config {
    config::Config::default()
}

impl App {
    pub fn new(config: Option<crate::app::config::Config>) -> Self {
        // setup sdl
        let sdl_context = sdl3::init().unwrap();
        // print SDL version
        let version = sdl3::version::version();
        println!(
            "SDL version: {}.{}.{}",
            version.major, version.minor, version.patch
        );
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
        let driver = video_subsystem.current_video_driver();
        println!("Using video driver: {}", driver);
        let display_id = video_subsystem.get_primary_display_id();
        let display_mode = video_subsystem.current_display_mode(display_id).unwrap();
        let window_width = display_mode.w as u32;
        let window_height = display_mode.h as u32;
        println!(
            "Display {} is {}x{}",
            display_index, window_width, window_height
        );
        let window = video_subsystem
            .window("ProjectM", window_width, window_height)
            .opengl()
            .maximized()
            .position_centered()
            // .allow_highdpi()
            .build()
            .expect("could not initialize video subsystem");

        // create openGL context
        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();

        // initialize projectM
        let pm = Rc::new(ProjectMWrapped::create());

        // and a preset playlist
        let playlist = projectm::playlist::Playlist::create(&pm);

        // get/set window size
        let (width, height) = window.size(); // TODO: handle pixel density https://github.com/libsdl-org/SDL/blob/main/docs/README-highdpi.md
        pm.set_window_size(width.try_into().unwrap(), height.try_into().unwrap());

        // initialize audio
        let audio = audio::Audio::new(&sdl_context, Rc::clone(&pm));

        Self {
            pm,
            playlist,
            sdl_context,
            window,
            config: config.unwrap_or_else(default_config),
            audio,
            _gl_context: gl_context, // keep this around to keep the context alive
        }
    }

    pub fn init(&mut self) {
        // load config
        self.load_config(&self.config);

        // initialize audio
        self.audio.init(self.get_frame_rate());
    }
}
