use std::io::Cursor;

use failure::ResultExt;

pub struct Player {
    sound_file: SoundFile,
}

impl Player {
    pub fn new(file_data: SoundFile) -> Player {
        Player {
            sound_file: file_data,
        }
    }

    pub fn play(&self) -> Result<(), failure::Error> {
        let device = rodio::default_output_device().expect("failed to find a sound device");
        let sink = rodio::play_once(&device, Cursor::new(self.sound_file.get_bytes()))
            .context("filed to play sound")?;
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
            SoundFile::BELL => include_bytes!("bell.mp3").to_vec()
        }
    }
}


