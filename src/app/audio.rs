use crate::app::App;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

pub struct Audio {
    app: &App,
    audio_subsystem: sdl2::AudioSubsystem,
    audio_device_index: u8,
    // device_list: Vec<sdl2::audio::AudioDevice>,
}

impl Audio {
    pub fn new(app: &App) -> Self {
        let audio_subsystem = app.sdl_context.audio().unwrap();

        Self {
            app,
            audio_subsystem,
            audio_device_index: 0,
        }
    }

    // pub fn get_device_list(mut self) -> Vec<sdl2::audio::AudioDevice> {
    //     let audio_subsystem = sdl_context.audio().unwrap();
    //     let device_list = audio_subsystem
    //         .capture_devices()
    //         .expect("could not get audio device list");
    //     for (i, device) in device_list.enumerate() {
    //         println!("{}: {}", i, device.name());
    //     }
    // }

    pub fn begin_audio_capture(mut self) {
        let frame_rate = self.app.config.frame_rate.unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: None,
            samples: Some(512), // should be 1 frame
        };
    }
}
