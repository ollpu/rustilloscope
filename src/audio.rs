extern crate portaudio as pa;
extern crate time;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 256;
const CHANNELS: i32 = 2;
const INTERLEAVED: bool = true;

pub fn run() -> Result<(), pa::Error> {
    let pa = pa::PortAudio::new()?;

    println!("PortAudio:");
    println!("version: {}", pa.version());
    println!("version text: {:?}", pa.version_text());
    println!("host count: {}", try!(pa.host_api_count()));

    let mut settings =
        pa.default_input_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES)?;

    let callback = move |args: pa::InputStreamCallbackArgs<f32>| {
        let pa::InputStreamCallbackArgs { buffer, .. } = args;
        println!("{}", buffer[0]);
        pa::Continue
    };

    // Construct a stream with input and output sample types of f32.
    let mut stream = pa.open_non_blocking_stream(settings, callback)?;
    stream.start()?;

    use std::{thread, time};
    thread::sleep(time::Duration::from_millis(5000));

    stream.stop()?;

    Ok(())
}
