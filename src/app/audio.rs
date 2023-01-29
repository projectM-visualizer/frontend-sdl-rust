use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

pub struct Audio {
    audio_subsystem: sdl2::AudioSubsystem,
    audio_device_index: u8,
    // device_list: Vec<sdl2::audio::AudioDevice>,
}

impl Audio {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();

        Self {
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

    pub fn begin_audio_capture(self, frame_rate: u32) {
        let sample_rate: u32 = 44100;

        // should be enough for 1 frame
        let buffer_size = (sample_rate / frame_rate) as u16;

        let desired_spec = AudioSpecDesired {
            freq: Some(sample_rate.try_into().unwrap()),
            channels: None,
            samples: Some(buffer_size),
        };

        // let audio_device = self
        //     .audio_subsystem
        //     .open_capture(None, &desired_spec, |spec| {
        //         // initialize the audio callback
        //         AudioCallback {
        //             spec,
        //             buffer_size,
        //             buffer: vec![0; buffer_size as usize],
        //             position: 0,
        //         }
        //     })
        //     .unwrap();
    }
}
