use projectm_rs::core::{projectm, projectm_handle};
use projectm_rs::playlist;
use sdl2::video::GLProfile;

pub mod main_loop;

pub struct Config {
    pub preset_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        // default preset path
        Self {
            // load from home dir or w/e
            preset_path: Some(String::from("/usr/local/share/projectM/presets")),
        }
    }
}

pub struct App {
    pm: projectm_handle,
    playlist: playlist::Playlist,
    sdl_context: sdl2::Sdl,
    gl_context: sdl2::video::GLContext,
    window: sdl2::video::Window,
}

impl App {
    pub fn new(config: Option<Config>) -> Self {
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
        let mut playlist = projectm_rs::playlist::Playlist::create(pm);

        // get/set window size
        let (width, height) = window.drawable_size(); // highDPI aware
        projectm::set_window_size(pm, width.try_into().unwrap(), height.try_into().unwrap());

        let mut this = Self {
            pm,
            playlist,
            sdl_context,
            gl_context,
            window,
        };

        // read config
        if let Some(config) = config {
            this.load_config(config);
        }

        this
    }

    pub fn load_config(&mut self, config: Config) {
        // load presets if provided
        if let Some(preset_path) = config.preset_path {
            self.add_preset_path(&preset_path);
        }
    }

    /// Add presets to the playlist recursively skipping duplicates.
    pub fn add_preset_path(&mut self, preset_path: &str) {
        self.playlist.add_path(preset_path, true);
        println!("added preset path: {}", preset_path);
        println!("playlist size: {}", self.playlist.len());
    }
}
