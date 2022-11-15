pub fn sine(size: usize) -> Vec<f32> {
    let mut wave_table = Vec::with_capacity(size);

    for n in 0..size {
        let phase = 2.0 * std::f32::consts::PI * n as f32 / size as f32;
        wave_table.push((phase).sin());
    }

    return wave_table;
}

pub fn square(size: usize) -> Vec<f32> {
    let mut wave_table = Vec::with_capacity(size);

    for n in 0..size {
        let phase = 2.0 * std::f32::consts::PI * n as f32 / size as f32;
        wave_table.push(if phase < std::f32::consts::PI { 1 } else { -1 } as f32)
    }

    return wave_table;
}

pub fn triangle(size: usize) -> Vec<f32> {
    let mut wave_table = Vec::with_capacity(size);

    for n in 0..size {
        let twopi = 2.0 * std::f32::consts::PI;
        let phase = twopi * n as f32 / size as f32;
        let val = 2.0 * (phase * (1.0 / twopi)) - 1.0;

        wave_table.push(if val < 0.0 { -val } else { 2.0 * (val - 0.5) } as f32)
    }

    return wave_table;
}

pub fn sawtooth(size: usize) -> Vec<f32> {
    let mut wave_table = Vec::with_capacity(size);

    for n in 0..size {
        let twopi = 2.0 * std::f32::consts::PI;
        let phase = twopi * n as f32 / size as f32;

        wave_table.push(2.0 * (phase * (1.0 / twopi)) - 1.0 as f32)
    }

    return wave_table;
}
