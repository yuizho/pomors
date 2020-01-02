use std::io::BufReader;

use failure::ResultExt;
use rodio::Device;

pub struct Player {
    device: Device,
}

impl Player {
    pub fn new() -> Player {
        Player {
            device: rodio::default_output_device()
                .expect("failed to find a sound device"),
        }
    }

    pub fn play(&self, sound_file: impl MetaData) -> Result<(), failure::Error> {
        let file = std::fs::File::open(sound_file.path()).context("no sound file")?;
        let sink = rodio::play_once(&self.device, BufReader::new(file))
            .context("filed to play sound")?;
        sink.detach();
        Ok(())
    }
}

pub trait MetaData {
    fn path(&self) -> &str;
}

pub enum SoundFile {
    BELL,
}

impl MetaData for SoundFile {
    fn path(&self) -> &str {
        match self {
            SoundFile::BELL => "res/bell.mp3"
        }
    }
}


