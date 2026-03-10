use std::thread::sleep;
use std::time::Duration;
use rodio::Source;
use synthesizer::WavetableOscillator;

fn main() {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 /
            wave_table_size as f32).sin());
    }

    let mut oscillator = WavetableOscillator::new(44100, wave_table.clone());
    oscillator.set_frequency(440.0);


    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("open default audio stream");

    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    
    // rodio::OutputStream::mixer(&stream_handle);

    // stream_handle.mixer().add(oscillator);

    sink.append(oscillator.take_duration(Duration::from_secs(2)));


    // sink.append(oscillator2.take_duration(Duration::from_secs(1)));

    sink.sleep_until_end();
    // sleep(Duration::from_secs(5));
}