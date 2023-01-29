use projectm_rs::core::projectm_handle;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

use super::config::FrameRate;

type AudioDeviceIndex = u32;
type SampleFormat = f32; // format of audio samples

pub struct AudioCaptureDevice {
    name: String,
    index: AudioDeviceIndex,
}

pub struct Audio {
    audio_subsystem: sdl2::AudioSubsystem,
    device_index: AudioDeviceIndex,
    is_capturing: bool,
    frame_rate: Option<FrameRate>,
    capturing_device: Option<AudioDevice<AudioCaptureCallback>>,
    projectm: projectm_handle,
}

impl Audio {
    pub fn new(sdl_context: &sdl2::Sdl, projectm: projectm_handle) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();

        Self {
            is_capturing: false,
            audio_subsystem,
            device_index: 0,
            frame_rate: None,
            capturing_device: None,
            projectm,
        }
    }

    pub fn init(&mut self, frame_rate: FrameRate) {
        self.list_devices();

        self.frame_rate = frame_rate.into();

        #[cfg(not(feature = "dummy_audio"))]
        self.begin_audio_capture();
    }

    pub fn list_devices(&self) {
        let devices = self.get_device_list();

        println!("Audio Devices:");
        for device in devices {
            println!(" - {}: {}", device.index, device.name);
        }
    }

    pub fn set_device(&mut self, device_index: AudioDeviceIndex) {
        self.stop_audio_capture();
        self.device_index = device_index;
        self.begin_audio_capture();
    }

    pub fn get_current_device_name(&self) -> String {
        let device_name = self
            .audio_subsystem
            .audio_capture_device_name(self.device_index)
            .expect("could not get audio device");
        device_name
    }

    pub fn open_next_device(&mut self) {
        let device_list = self.get_device_list();
        let current_device_index = self.device_index;

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

            device_list.push(AudioCaptureDevice {
                name: device_name,
                index: i,
            });
        }

        device_list
    }

    pub fn begin_audio_capture<'a>(&'a mut self) {
        let sample_rate: u32 = 44100;
        let frame_rate = self.frame_rate.unwrap();

        // should be enough for 1 frame
        let buffer_size = (sample_rate / frame_rate) as u16;

        let desired_spec = AudioSpecDesired {
            freq: Some(sample_rate.try_into().unwrap()),
            channels: Some(2),
            samples: Some(buffer_size),
        };

        let audio_device = self
            .audio_subsystem // sdl
            .open_capture(None, &desired_spec, |_spec| {
                println!(
                    "Beginning audio capture for device {}",
                    self.get_current_device_name()
                );

                // print spec
                println!("Audio Spec: {:?}", _spec);

                // return callback fn
                AudioCaptureCallback {
                    pm: self.projectm,
                    // spec,
                    // buffer_size,
                    // buffer: vec![0; buffer_size as usize],
                    // position: 0,
                }
            })
            .unwrap();

        // take ownership of device
        self.capturing_device = Some(audio_device);
        self.is_capturing = true;

        // play device
        self.capturing_device.as_mut().unwrap().resume();
    }

    pub fn stop_audio_capture(&mut self) {
        let current_device_name = self.get_current_device_name();
        println!("Stopping audio capture for device {}", current_device_name);

        println!(
            "current capture device: {:?}",
            self.capturing_device.as_ref().unwrap().status()
        );

        self.is_capturing = false;
        // drop(self.capturing_device); // stop capturing
        self.capturing_device = None;
    }
}

struct AudioCaptureCallback {
    pm: projectm_handle,
    // spec: sdl2::audio::AudioSpec,
    // buffer_size: SampleFormat,
    // buffer: Vec<u8>,
    // position: usize,
}
unsafe impl Send for AudioCaptureCallback {}
unsafe impl Sync for AudioCaptureCallback {}

impl AudioCallback for AudioCaptureCallback {
    type Channel = SampleFormat;

    // we are receiving some chunk of audio data
    // we need to pass it to projectm
    fn callback(&mut self, out: &mut [SampleFormat]) {
        let pm = self.pm;
        projectm_rs::core::projectm::pcm_add_float(pm, out.to_vec(), 2);
    }
}
