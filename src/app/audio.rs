use projectm::core::ProjectM;
use sdl3::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

use super::config::FrameRate;
use super::ProjectMWrapped;

use std::rc::Rc;

type AudioDeviceIndex = u32;
type SampleFormat = f32; // format of audio samples

pub struct AudioCaptureDevice {
    name: String,
    index: AudioDeviceIndex,
}

pub struct Audio {
    audio_subsystem: sdl3::AudioSubsystem,
    device_index: AudioDeviceIndex,
    is_capturing: bool,
    frame_rate: Option<FrameRate>,
    capturing_device: Option<Box<AudioDevice<AudioCaptureCallback>>>,
    projectm: ProjectMWrapped,
}

/// Wrapper around the audio subsystem to capture audio and pass it to projectM.
impl Audio {
    pub fn new(sdl_context: &sdl3::Sdl, projectm: ProjectMWrapped) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();
        println!(
            "Using audio driver: {}",
            audio_subsystem.current_audio_driver()
        );

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
        self.begin_audio_capture(0);
    }

    pub fn list_devices(&self) {
        let devices = self.get_device_list();

        println!("Audio Devices:");
        for device in devices {
            println!(" - {}: {}", device.index, device.name);
        }
    }

    /// Start capturing audio from device_index.
    pub fn capture_device(&mut self, device_index: AudioDeviceIndex) {
        self.stop_audio_capture();
        println!("Capturing audio from device {}", device_index);
        self.device_index = device_index;
        self.begin_audio_capture(device_index);
        println!("Capturing audio from device {}", device_index);
    }

    pub fn get_device_name(&self, device_index: AudioDeviceIndex) -> String {
        self.audio_subsystem
            .audio_capture_device_name(device_index)
            .expect("could not get audio device")
    }

    /// Select a new audio device and start capturing audio from it.
    pub fn open_next_device(&mut self) {
        let device_list = self.get_device_list();
        let current_device_index = self.device_index;

        let next_device_index = (current_device_index + 1) % device_list.len() as AudioDeviceIndex;
        println!("Opening next device: {}", next_device_index);
        self.capture_device(next_device_index);
    }

    fn get_device_list(&self) -> Vec<AudioCaptureDevice> {
        let audio_subsystem = &self.audio_subsystem;

        let num_devices = audio_subsystem
            .num_audio_capture_devices()
            .expect("could not get number of audio devices");

        (0..num_devices)
            .map(|i| {
                let device_name = audio_subsystem
                    .audio_capture_device_name(i)
                    .expect("could not get audio device");
                AudioCaptureDevice {
                    name: device_name,
                    index: i,
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn begin_audio_capture(&mut self, device_index: AudioDeviceIndex) {
        let sample_rate: u32 = 44100;
        let frame_rate = self.frame_rate.unwrap();

        // how many samples to capture at a time
        // should be enough for 1 frame or less
        // should not be larger than max_samples / channels
        let max_samples: usize = ProjectM::pcm_get_max_samples().try_into().unwrap();
        let samples_per_frame = (sample_rate / frame_rate) as usize;
        let buffer_size = std::cmp::min(max_samples / 2, samples_per_frame);
        println!("Buffer size: {}", buffer_size);

        let desired_spec = AudioSpecDesired {
            freq: Some(sample_rate.try_into().unwrap()),
            channels: Some(2),
            samples: Some(buffer_size.try_into().unwrap()),
        };

        // open audio device for capture
        let device_name = self.get_device_name(device_index);
        println!("Opening audio device: {}", device_name);
        let audio_device = match self
            .audio_subsystem // sdl
            .open_capture(device_name.as_str(), &desired_spec, |_spec| {
                println!("Beginning audio capture for device {}", device_name);

                // print spec
                println!("Audio Spec: {:?}", _spec);

                // return callback fn
                AudioCaptureCallback {
                    pm: Rc::clone(&self.projectm),
                }
            }) {
            Ok(device) => device,
            Err(e) => {
                println!("Error opening audio device: {}", e);
                return;
            }
        };

        // start capturing
        audio_device.resume();

        // take ownership of device
        self.capturing_device = Some(Box::new(audio_device));
        self.is_capturing = true;
    }

    pub fn stop_audio_capture(&mut self) {
        let current_device_name = self.get_device_name(self.device_index);
        println!("Stopping audio capture for device {}", current_device_name);

        println!(
            "Current capture device status: {:?}",
            self.capturing_device.as_ref().unwrap().status()
        );

        // take ownership of device
        // capture device will be dropped when this function returns
        // and the audio callback will stop being called
        let device = self.capturing_device.take().unwrap();
        device.pause();

        println!("Device paused");

        self.is_capturing = false;

        println!("Stopped audio capture");
        drop(device);
        println!("Dropped audio device");
    }
}

struct AudioCaptureCallback {
    // we need to keep a reference to the projectm instance to
    // add the audio data to it
    pm: ProjectMWrapped,
}
unsafe impl Send for AudioCaptureCallback {}
unsafe impl Sync for AudioCaptureCallback {}

impl AudioCallback for AudioCaptureCallback {
    type Channel = SampleFormat;

    // we are receiving some chunk of audio data
    // we need to pass it to projectm
    fn callback(&mut self, out: &mut [SampleFormat]) {
        self.pm.pcm_add_float(out.to_vec(), 2);
    }
}
