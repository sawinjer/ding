use std::io::{Cursor};
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    let done_sound_cursor = Cursor::new(include_bytes!("./done.mp3"));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let done_source = Decoder::new_mp3(done_sound_cursor).unwrap();
    
    sink.append(done_source);
    sink.sleep_until_end();
}
