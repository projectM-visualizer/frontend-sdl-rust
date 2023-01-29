use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

type AudioDeviceIndex = u32;

pub struct AudioCaptureDevice {
    name: String,
    index: AudioDeviceIndex,
}

pub struct Audio<'a> {
    audio_subsystem: sdl2::AudioSubsystem,
    audio_device_index: AudioDeviceIndex, // device_list: Option<Vec<sdl2::audio::AudioDevice>>,
    frame_rate: Option<u32>,
    capturing_device: Option<AudioDevice<AudioCaptureCallback>>,
}

impl Audio<'_> {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();

        Self {
            audio_subsystem,
            audio_device_index: 0,
            frame_rate: None,
            capturing_device: None,
        }
    }

    pub fn init(&mut self, frame_rate: u32) {
        self.frame_rate = frame_rate.into();
        self.begin_audio_capture();
    }

    pub fn list_devices(&self) {
        self.get_device_list();
    }

    pub fn set_device(&mut self, device_index: AudioDeviceIndex) {
        self.stop_audio_capture();
        self.audio_device_index = device_index;
        self.begin_audio_capture();
    }

    pub fn get_current_device_name(&self) -> String {
        let device_name = self
            .audio_subsystem
            .audio_capture_device_name(self.audio_device_index)
            .expect("could not get audio device");
        device_name
    }

    pub fn open_next_device(&mut self) {
        let device_list = self.get_device_list();
        let current_device_index = self.audio_device_index;

        let next_device_index = current_device_index + 1 % device_list.len() as AudioDeviceIndex;
        self.set_device(next_device_index);
    }

    fn get_device_list(&self) -> Vec<AudioCaptureDevice> {
        let audio_subsystem = &self.audio_subsystem;

        let num_devices = audio_subsystem
            .num_audio_capture_devices()
            .expect("could not get number of audio devices");

        let mut device_list: Vec<AudioCaptureDevice> = vec![];

        for i in 0..num_devices {
            let device_name = audio_subsystem
                .audio_capture_device_name(i)
                .expect("could not get audio device");
            println!("{}: {}", i, device_name);

            device_list.push(AudioCaptureDevice {
                name: device_name,
                index: i,
            });
        }

        device_list
    }

    pub fn begin_audio_capture<'a>(&'a mut self) {
        let sample_rate: u32 = 44100;
        let frame_rate: u32 = self
            .frame_rate
            .expect("frame rate not set, call init() pls");

        // should be enough for 1 frame
        let buffer_size = (sample_rate / frame_rate) as u16;

        let desired_spec = AudioSpecDesired {
            freq: Some(sample_rate.try_into().unwrap()),
            channels: None,
            samples: Some(buffer_size),
        };

        // let mut audio_device: &'a AudioDevice<AudioCaptureCallback> = &self
        let mut audio_device = self
            .audio_subsystem // sdl
            .open_capture(None, &desired_spec, |spec| {
                println!(
                    "Beginning audio capture for device {}",
                    self.get_current_device_name()
                );
                // return callback fn
                AudioCaptureCallback {
                    spec,
                    buffer_size,
                    buffer: vec![0; buffer_size as usize],
                    position: 0,
                }
            })
            .unwrap();

        self.capturing_device = Some(audio_device);
    }

    pub fn stop_audio_capture(&self) {
        let current_device_name = self.get_current_device_name();
        println!("Stopping audio capture for device {}", current_device_name);

        let current_device = self.audio_device_index;
        drop(self.capturing_device);
    }
}

struct AudioCaptureCallback {
    spec: sdl2::audio::AudioSpec,
    buffer_size: u16,
    buffer: Vec<u8>,
    position: usize,
}

impl AudioCallback for AudioCaptureCallback {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        // let buffer = &mut self.buffer;
        // let position = &mut self.position;
        // let buffer_size = self.buffer_size;

        // let mut i = 0;
        // while i < out.len() {
        //     out[i] = buffer[*position];
        //     *position += 1;
        //     if *position >= buffer_size as usize {
        //         *position = 0;
        //     }
        //     i += 1;
        // }
    }
}
