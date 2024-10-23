use projectm::core::ProjectM;
use sdl3::audio::{AudioDevice, AudioDeviceID, AudioRecordingCallback, AudioSpec, AudioStreamWithCallback};

use super::config::FrameRate;
use super::ProjectMWrapped;

use std::rc::Rc;

// let the user cycle through audio devices
type AudioDeviceIndex = usize;

type SampleFormat = f32; // format of audio samples


pub struct Audio {
    audio_subsystem: sdl3::AudioSubsystem,

    current_recording_device: Option<AudioDevice>,
    capture_stream: Option<Box<AudioStreamWithCallback<AudioCaptureCallback>>>,
    is_capturing: bool,

    frame_rate: Option<FrameRate>,
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
            current_recording_device: None,
            frame_rate: None,
            capture_stream: None,
            projectm,
        }
    }

    pub fn init(&mut self, frame_rate: FrameRate) {
        self.list_devices();

        self.frame_rate = frame_rate.into();

        #[cfg(not(feature = "dummy_audio"))]
        self.begin_audio_capture(self.get_default_recording_device().id());
    }

    pub fn list_devices(&self) {
        let devices = self.get_device_list();

        println!("Audio Devices:");
        for device in devices {
            println!(" - {} [{}]", device.name(), device.id());
        }
    }

    /// Start capturing audio from device_id.
    pub fn capture_device(&mut self, device_id: AudioDeviceID) {
        self.stop_audio_capture();
        self.begin_audio_capture(device_id);
    }

    fn get_default_recording_device(&self) -> AudioDevice {
        self.audio_subsystem.default_recording_device()
    }


    // pub fn get_device_at_index(&mut self, device_index: AudioDeviceIndex) -> AudioDevice {
    //         let devices = self.get_device_list();
    //     if device_index < devices.len() as AudioDeviceIndex {
    //         // should it return a ref or make a new AudioDevice?
    //         AudioDevice::clone(&devices[device_index])
    //     } else {
    //         if devices.is_empty() {
    //             panic!("No audio recording devices found");
    //         }
    //         self.device_index = 0;
    //         AudioDevice::clone(&devices[0])
    //     }
    // }

    /// Select a new audio device and start capturing audio from it.
    pub fn open_next_device(&mut self) {
        let device_list = self.get_device_list();
        let current_device = self.current_recording_device.as_ref().unwrap();
        let current_device_id = current_device.id();
        let current_device_index = device_list.iter().position(|d| d.eq(&current_device_id)).unwrap();
        let next_device_index = (current_device_index + 1) % device_list.len();
        let next_device_id = device_list[next_device_index];
        self.capture_device(next_device_id);
    }

    fn get_device_list(&self) -> Vec<AudioDeviceID> {
        let audio_subsystem = &self.audio_subsystem;

        audio_subsystem.audio_recording_device_ids().unwrap()
    }

    pub fn begin_audio_capture(&mut self, device_id: AudioDeviceID) {
        let sample_rate: u32 = 44100;
        let frame_rate = self.frame_rate.unwrap();

        // how many samples to capture at a time
        // should be enough for 1 frame or less
        // should not be larger than max_samples / channels
        let max_samples: usize = ProjectM::pcm_get_max_samples().try_into().unwrap();
        let samples_per_frame = (sample_rate / frame_rate) as usize;
        let buffer_size = std::cmp::min(max_samples / 2, samples_per_frame);
        println!("Capturing audio from device {}", device_id.name());
        println!("Buffer size: {}", buffer_size);

        let desired_spec = AudioSpec {
            freq: Some(sample_rate.try_into().unwrap()),
            channels: Some(2),
            format: Some(sdl3::audio::AudioFormat::f32_sys()),
        };

        // open audio device for capture
        let device = AudioDevice::new(device_id);
        let audio_stream =
            device
                // move this to open_recording_device
                .open_recording_stream_with_callback(
                    &desired_spec,
                    AudioCaptureCallback {
                        pm: Rc::clone(&self.projectm),
                    },
                ).unwrap();


        // start capturing
        audio_stream.resume();

        // take ownership of device
        self.capture_stream = Some(Box::new(audio_stream));
        self.is_capturing = true;
    }

    pub fn recording_device_name(&self) -> Option<String> {
        self.current_recording_device.as_ref().map(|device| device.name())
    }

    pub fn stop_audio_capture(&mut self) {
        if self.current_recording_device.is_none() {
            return;
        }

        let current_device_name = self.recording_device_name();
        println!("Stopping audio capture for device {}", current_device_name.unwrap_or("unknown".to_string()));

        // println!(
        //     "Current capture device status: {:?}",
        //     self.capture_stream.as_ref().unwrap().status()
        // );

        // take ownership of device
        // capture device will be dropped when this function returns
        // and the audio callback will stop being called
        let device = self.capture_stream.take().unwrap();
        device.pause();

        self.is_capturing = false;
        drop(device);
    }
}

struct AudioCaptureCallback {
    // we need to keep a reference to the projectm instance to
    // add the audio data to it
    pm: ProjectMWrapped,
}
unsafe impl Send for AudioCaptureCallback {}
unsafe impl Sync for AudioCaptureCallback {}

impl AudioRecordingCallback<SampleFormat> for AudioCaptureCallback {
    // we are receiving some chunk of audio data
    // we need to pass it to projectm
    fn callback(&mut self, out: &[SampleFormat]) {
        println!("Received {} samples", out.len());
        let mut out = out;
        let max_samples = ProjectM::pcm_get_max_samples() as usize;
        if (out.len() > max_samples) {
            // remove some samples
            out = &out[..max_samples];
        }
        self.pm.pcm_add_float(out.to_vec(), 2);
    }
}
