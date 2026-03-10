use std::sync::Arc;
use std::time::Duration;
use rodio::Source;
use crate::synth::WavetableOscillator;

pub struct Synth {
    sample_rate: u32,
    wavetable: Arc<Vec<f32>>,
    _stream: rodio::OutputStream,
    sink: rodio::Sink,
}

impl Synth {
    pub fn new(sample_rate: u32, wavetable: Vec<f32>) -> Self {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");

        let sink = rodio::Sink::connect_new(stream.mixer());

        Synth {
            sample_rate,
            wavetable: Arc::new(wavetable),
            _stream: stream,
            sink
        }
    }

    pub fn append(&self, freq: f32, duration: Duration) {
        let mut oscillator = WavetableOscillator::new(self.sample_rate, Arc::clone(&self.wavetable));
        oscillator.set_frequency(freq);

        self.sink.append(oscillator.take_duration(duration));
    }

    pub fn play(&self) {
        self.sink.sleep_until_end();
    }
}