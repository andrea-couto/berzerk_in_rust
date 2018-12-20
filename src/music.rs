//from rust-adl2 example

extern crate sdl2;

use music::sdl2::audio::{AudioCallback, AudioSpecDesired,AudioSpecWAV,AudioCVT};
use std::time::Duration;
use std::borrow::Cow;
use std::path::{PathBuf, Path};
use std::env::*;
use std::thread::*;


struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            *dst = (*self.data.get(self.pos).unwrap_or(&0) as f32 * self.volume) as u8;
            self.pos += 1;
        }
    }
}

// I tried everything i could think of to pass a string to this function and
// I couldnt get past the lifetime errors
/// play will play sounds depending on the u32 that is passed to the function
/// assets are grabbed from the assets folder
pub fn play(sound: u32) {

    let player_shoot = "./assets/shoot_player.wav";
    let player_die = "./assets/lost_life.wav";
    let enemy_die = "./assets/enemy_lost_life.wav";
    let enemy_shoot = "./assets/shoot_enemy.wav";
    let player_die_final = "./assets/player_die.wav";

    let file_to_play: &str;

    match sound {
        0 => file_to_play = player_shoot,
        1=> file_to_play = player_die,
        2=> file_to_play = enemy_die,
        3=> file_to_play = enemy_shoot,
        4=> file_to_play = player_die_final,
        _ => file_to_play = player_shoot
    }

    let wav_file : Cow<'static, Path> = match args().nth(1) {
        None => Cow::from(Path::new(file_to_play)),
        Some(s) => Cow::from(PathBuf::from(s))
    };
    let sdl_context = sdl2::init().unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: None      // default
    };
    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(wav_file)
            .expect("Could not load test WAV file");
        let cvt = AudioCVT::new(
                wav.format, wav.channels, wav.freq,
                spec.format, spec.channels, spec.freq)
            .expect("Could not convert WAV file");
        let data = cvt.convert(wav.buffer().to_vec());
        Sound {
            data: data,
            volume: 0.25,
            pos: 0,
        }
    }).unwrap();
    device.resume();
    sleep(Duration::from_millis(1_000));
}