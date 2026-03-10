use std::time::Duration;
use synthesizer::synth::wavetable;
use synthesizer::synth::Synth;

fn main() {
    // Create sine wave oscillator
    let wave_table_size = 64;
    let wave_table = wavetable::sine(wave_table_size);

    let new_synth = Synth::new(44100, wave_table);

    for i in 0..12 {
        new_synth.append(440.0 * 2f32.powf(i as f32 / 12.0), Duration::from_millis(300));
    }

    new_synth.play();
}