use std::io::Cursor;

use failure::ResultExt;
use rodio::Device;

pub struct Player {
    device: Device,
}

impl Player {
    pub fn new() -> Player {
        Player {
            device: rodio::default_output_device().expect("failed to find a sound device"),
        }
    }

    pub fn play(&self, sound_file: impl FileData) -> Result<(), failure::Error> {
        let sink = rodio::play_once(&self.device, Cursor::new(sound_file.get_bytes()))
            .context("filed to play sound")?;
        sink.set_volume(0.1);
        sink.detach();
        Ok(())
    }
}

pub trait FileData {
    fn get_bytes(&self) -> Vec<u8>;
}

pub enum SoundFile {
    BELL,
}

impl FileData for SoundFile {
    fn get_bytes(&self) -> Vec<u8> {
        match self {
            SoundFile::BELL => include_bytes!("bell.mp3").to_vec(),
        }
    }
}
