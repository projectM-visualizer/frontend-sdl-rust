use crate::app::config::Config;
use projectm::core::ProjectM;
use sdl3::video::GLProfile;
use std::convert::TryInto;
use std::rc::Rc;

pub mod audio;
pub mod config;
pub mod main_loop;
pub mod playlist;
pub mod video;

pub type ProjectMWrapped = Rc<ProjectM>;

/// Application state
pub struct App {
    pm: ProjectMWrapped,
    playlist: projectm::playlist::Playlist,
    sdl_context: sdl3::Sdl,
    window: sdl3::video::Window,
    config: config::Config,
    audio: audio::Audio,
    _gl_context: sdl3::video::GLContext,
}

impl App {
    pub fn new(config: Option<Config>) -> Self {
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
        let mut window = video_subsystem
            .window("ProjectM", 1024, 768)
            .build()
            .expect("could not initialize video subsystem");

        // create openGL context
        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();

        // initialize projectM
        let pm = Rc::new(ProjectM::create());

        // and a preset playlist
        let playlist = projectm::playlist::Playlist::create(&pm);

        // make window full-size
        let primary_display = video_subsystem.get_primary_display().unwrap();
        let display_bounds = primary_display.get_usable_bounds().unwrap();
        window
            .set_size(display_bounds.width(), display_bounds.height())
            .unwrap();
        window.set_position(WindowPos::Centered, WindowPos::Centered);
        window
            .set_display_mode(None)
            .expect("could not set display mode");

        // initialize audio
        let audio = audio::Audio::new(&sdl_context, Rc::clone(&pm));

        Self {
            pm,
            playlist,
            sdl_context,
            window,
            config: config.unwrap_or_else(Config::default),
            audio,
            _gl_context: gl_context, // keep this around to keep the context alive
        }
    }

    pub fn init(&mut self) {
        // load config
        self.load_config(&self.config);

        // initialize audio
        self.audio.init(self.get_frame_rate());

        self.update_projectm_window_size();
    }
}
