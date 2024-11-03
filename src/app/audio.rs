use sdl3::audio::{AudioDevice, AudioDeviceID, AudioSpec, AudioStream};

use super::config::FrameRate;
use super::ProjectMWrapped;

type SampleFormat = f32; // Format of audio samples

pub struct Audio {
    audio_subsystem: sdl3::AudioSubsystem,
    recording_stream: Option<Box<AudioStream>>,
    is_capturing: bool,
    frame_rate: Option<FrameRate>,
    projectm: ProjectMWrapped,
    current_device_id: Option<AudioDeviceID>,
    current_device_name: Option<String>, // Store device name for comparison
}

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
            current_device_name: None,
            recording_stream: None,
            projectm,
        }
    }

    pub fn init(&mut self, frame_rate: FrameRate) {
        self.list_devices();

        self.frame_rate = Some(frame_rate);

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
        // Stop capturing from current stream/device
        self.stop_audio_capture();

        let sample_rate: u32 = 44100;

        let desired_spec = AudioSpec {
            freq: Some(sample_rate as i32),
            channels: Some(2),
            format: Some(sdl3::audio::AudioFormat::f32_sys()), // Assuming F32SYS is the correct format
        };

        // Open audio device for capture (use default device if none specified)
        let device_id = device_id
            .or_else(|| Some(self.get_default_recording_device().id()))
            .unwrap(); // Ensure device_id is Some

        let audio_stream = match AudioStream::open_device_stream(device_id, Some(&desired_spec)) {
            Ok(stream) => stream,
            Err(e) => {
                println!("Failed to open audio stream: {}", e);
                return;
            }
        };
        println!("Capturing audio from device {:?}", audio_stream);

        // Get the actual device ID and name from the stream
        let actual_device_id = audio_stream.device_id();
        let actual_device_name = audio_stream.device_name();

        if actual_device_id.is_none() {
            println!("Failed to get device ID from audio stream: {:?}", audio_stream);
            return;
        }

        let actual_device_id = actual_device_id.unwrap();
        let actual_device_name = actual_device_name.unwrap_or_else(|| "unknown".to_string());

        // Start capturing
        if let Err(e) = audio_stream.resume() {
            println!("Failed to start audio capture: {}", e);
            return;
        }

        // Take ownership of the stream and store device information
        self.recording_stream = Some(Box::new(audio_stream));
        self.current_device_id = Some(actual_device_id);
        self.current_device_name = Some(actual_device_name);
        self.is_capturing = true;
    }

    fn get_default_recording_device(&self) -> AudioDevice {
        self.audio_subsystem.default_recording_device()
    }

    /// Select a new audio device and start capturing audio from it.
    pub fn open_next_device(&mut self) {
        let current_device_name = self.recording_device_name();

        let device_list = self.get_device_list();

        println!("Device list: {:?}", device_list);
        println!("Current device name: {:?}", current_device_name);

        // Find the index of the current device by name
        let current_device_index = current_device_name.as_ref().and_then(|name| {
            device_list
                .iter()
                .position(|d| d.name() == *name)
        });

        let current_device_index = current_device_index.unwrap_or_else(|| {
            println!("Current device not found in device list. Starting from the first device.");
            0
        });

        // Select next device index
        let next_device_index = (current_device_index + 1) % device_list.len();
        let next_device_id = device_list[next_device_index];

        println!(
            "Switching from device '{}' to '{}'",
            current_device_name.unwrap_or_else(|| "unknown".to_string()),
            next_device_id.name()
        );

        // Start capturing from next device
        self.begin_audio_capture(Some(next_device_id));
    }

    fn get_device_list(&self) -> Vec<AudioDeviceID> {
        self.audio_subsystem.audio_recording_device_ids().unwrap_or_else(|e| {
            println!("Failed to get audio device list: {}", e);
            Vec::new()
        })
    }

    pub fn recording_device_name(&self) -> Option<String> {
        self.current_device_name.clone()
    }

    pub fn stop_audio_capture(&mut self) {
        if let Some(stream) = self.recording_stream.take() {
            // Retrieve the device name before dropping the stream
            let current_device_name = stream.device_name().unwrap_or_else(|| "unknown".to_string());

            println!(
                "Stopping audio capture for device {}",
                current_device_name
            );

            // The recording device will be closed when the stream is dropped
            self.is_capturing = false;
            drop(stream);
        }
    }
}