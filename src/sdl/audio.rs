use chipsy::machine::audio::Audio;
use sdl2::{audio::{AudioCallback, AudioDevice, AudioSpec, AudioSpecDesired}, AudioSubsystem};

pub struct AudioPlayback<T>
where
    T: AudioCallback
{
    device: AudioDevice<T>
}

impl<T> AudioPlayback<T> 
where
    T: AudioCallback
{
    pub fn new<F: FnOnce(AudioSpec) -> T>(audio_subsystem: AudioSubsystem, desired_spec: &AudioSpecDesired, callback: F) -> AudioPlayback<T>
    {
        let device = audio_subsystem.open_playback(None, desired_spec, callback).unwrap();

        AudioPlayback {
            device
        }
    }
}

impl<T> Audio for AudioPlayback<T> 
where
    T: AudioCallback
{
    fn resume(&self){
        self.device.resume();
        // thread::sleep(duration);
        // self.pause();
    }

    fn pause(&self){
        self.device.pause()
    }
}

unsafe impl<T> Send for AudioPlayback<T>
where
    T: AudioCallback
{}