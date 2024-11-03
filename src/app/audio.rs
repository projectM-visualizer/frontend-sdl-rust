use sdl3::audio::{AudioDevice, AudioDeviceID, AudioSpec, AudioStream};
use std::io::Read;

use super::config::FrameRate;
use super::ProjectMWrapped;

// let the user cycle through audio devices
type AudioDeviceIndex = usize;

type SampleFormat = f32; // format of audio samples

pub struct Audio {
    audio_subsystem: sdl3::AudioSubsystem,
    recording_stream: Option<Box<AudioStream>>,
    is_capturing: bool,
    frame_rate: Option<FrameRate>,
    projectm: ProjectMWrapped,
    current_device_id: Option<AudioDeviceID>,
}

/// Wrapper around the audio subsystem to record audio and pass it to projectM.
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
            frame_rate: None,
            current_device_id: None,
            recording_stream: None,
            projectm,
        }
    }

    pub fn init(&mut self, frame_rate: FrameRate) {
        self.list_devices();

        self.frame_rate = frame_rate.into();

        #[cfg(not(feature = "dummy_audio"))]
        self.begin_audio_capture(None);
    }

    pub fn list_devices(&self) {
        let devices = self.get_device_list();

        println!("Audio Devices:");
        for device in devices {
            println!(" - {} [{}]", device.name(), device.id());
        }
    }

    /// Start capturing audio from device_id.
    pub fn begin_audio_capture(&mut self, device_id: Option<AudioDeviceID>) {
        // stop capturing from current stream/device
        self.stop_audio_capture();

        let sample_rate: u32 = 44100;
        let frame_rate = self.frame_rate.unwrap();

        // how many samples to capture at a time
        // should be enough for 1 frame or less
        // should not be larger than max_samples / channels
        // let max_samples: usize = ProjectM::pcm_get_max_samples().try_into().unwrap();
        // let samples_per_frame = (sample_rate / frame_rate) as usize;
        // let buffer_size = std::cmp::min(max_samples / 2, samples_per_frame);
        // println!("Buffer size: {}", buffer_size);

        let desired_spec = AudioSpec {
            freq: Some(sample_rate.try_into().unwrap()),
            channels: Some(2),
            format: Some(sdl3::audio::AudioFormat::f32_sys()),
        };

        // open audio device for capture (use default device if none specified)
        let device_id = (device_id
            .or_else(|| Some(self.get_default_recording_device().id()))
            .unwrap());

        let audio_stream = match AudioStream::open_device_stream(device_id, Some(&desired_spec)) {
            Ok(stream) => stream,
            Err(e) => {
                println!("Failed to open audio stream: {}", e);
                return;
            }
        };
        println!("Capturing audio from device {:?}", audio_stream);

        let device_id = audio_stream.device_id();
        if device_id.is_none() {
            println!("Failed to get begin audio capture: {:?}", audio_stream);
            return;
        }

        // start capturing
        audio_stream
            .resume()
            .expect("Failed to start audio capture");

        // take ownership of device
        self.recording_stream = Some(Box::new(audio_stream));
        self.current_device_id = device_id;
        self.is_capturing = true;
    }

    fn get_default_recording_device(&self) -> AudioDevice {
        self.audio_subsystem.default_recording_device()
    }

    /// Select a new audio device and start capturing audio from it.
    pub fn open_next_device(&mut self) {
        let current_device_id = self.current_device_id;
        if current_device_id.is_none() {
            self.begin_audio_capture(None);
            return;
        }
        let current_device_id = current_device_id.unwrap();

        // get list of devices
        let device_list = self.get_device_list();

        println!("Device list: {:?}", device_list);
        println!("Current device: {:?}", current_device_id);

        // find current device in list
        let mut current_device_index = device_list.iter().position(|d| d.eq(&current_device_id));

        // if current device not found, start from the beginning
        if current_device_index.is_none() {
            println!("Current device not found in device list");
            self.begin_audio_capture(None);
            return;
        }

        // select next device
        let next_device_index = (current_device_index.unwrap() + 1) % device_list.len();
        let next_device_id = device_list[next_device_index];

        // start capturing from next device
        self.begin_audio_capture(Some(next_device_id));
    }

    fn get_device_list(&self) -> Vec<AudioDeviceID> {
        let audio_subsystem = &self.audio_subsystem;

        audio_subsystem.audio_recording_device_ids().unwrap()
    }

    pub fn recording_device_name(&self) -> Option<String> {
        self.current_device_id.and_then(|id| Some(id.name()))
    }

    pub fn stop_audio_capture(&mut self) {
        let mut recording_stream = &mut self.recording_stream;
        if recording_stream.is_none() {
            return;
        }

        // take ownership of stream
        let stream = recording_stream.take().unwrap();

        let current_device_name = stream.device_name();
        println!(
            "Stopping audio capture for device {}",
            current_device_name.unwrap_or_else(|| "unknown".to_string())
        );

        // the recording device will be closed when the stream is dropped
        self.is_capturing = false;
        drop(stream);
    }
}

// struct AudioCaptureCallback {
//     // we need to keep a reference to the projectm instance to
//     // add the audio data to it
//     pm: ProjectMWrapped,
// }
// unsafe impl Send for AudioCaptureCallback {}
// unsafe impl Sync for AudioCaptureCallback {}
//
// impl AudioRecordingCallback<SampleFormat> for AudioCaptureCallback {
//     // we are receiving some chunk of audio data
//     // we need to pass it to projectm
//     fn callback(&mut self, out: &[SampleFormat]) {
//         println!("Received {} samples", out.len());
//         let mut out = out;
//         let max_samples = ProjectM::pcm_get_max_samples() as usize;
//         if (out.len() > max_samples) {
//             // remove some samples
//             out = &out[..max_samples];
//         }
//         self.pm.pcm_add_float(out.to_vec(), 2);
//     }
// }
