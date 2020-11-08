use std::io::Cursor;

pub fn play(sound_file: impl FileData) -> Result<(), failure::Error> {
    let (_stream, stream_handle) =
        rodio::OutputStream::try_default().expect("failed to find output device");
    let sink = rodio::Sink::try_new(&stream_handle).expect("failed to create sink");
    sink.append(
        rodio::Decoder::new(Cursor::new(sound_file.get_bytes())).expect("failed to playback sound"),
    );
    sink.set_volume(0.1);
    sink.sleep_until_end();
    Ok(())
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
