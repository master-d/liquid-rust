
use sdl2::audio::{AudioDevice,AudioSpecDesired};

pub struct Sound {
  device: AudioDevice,
  desired_spec: AudioSpecDesired
}

impl Default for Sound {
  fn default() -> Sound {
      let sdl_context = sdl2::init().unwrap();
      let audio_subsystem = sdl_context.audio().unwrap();

      let spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(2),  // mono
        samples: None       // default sample size
      };

      Sound {
        device: audio_subsystem,
        desired_spec: spec
      }
  }
}

impl Sound {
  fn play(&mut self, wavfile: &str) {
    let audio_wav = AudioSpecWAV::load_wav(wavfile).unwrap();
    let wavdata = WavData{ bytes: audio_wav.buffer().to_vec(), position: 0 };
      
    let ad = self.device.open_playback( None, &desired_spec, move |spec| {
      wavdata
    }).unwrap();
    ad.resume();
  }
}


struct WavData {
    bytes: Vec<u8>,
    position: usize
}

impl AudioCallback for WavData {
    type Channel = u8;

    fn callback(&mut self, data: &mut [u8]) {
        let (start, end) = (self.position, self.position + data.len());
        self.position += data.len();

        let audio_data = &self.bytes[start..end];
        for (src, dst) in audio_data.iter().zip(data.iter_mut()) {
            *dst = *src;
        }
    }
}