pub fn sine(size: usize) -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(size);

    for n in 0..size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 /
            size as f32).sin());
    }
    wave_table
}

pub fn saw(size: usize) -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(size);

    let p = 100.0;
    for n in 0..size {
        let t = n as f32;

        let x = 4.0 * ((t / p) - ((t / p) + 0.75).floor() + 0.25).abs() - 1.0;
        wave_table.push(x);
    }
    wave_table
}