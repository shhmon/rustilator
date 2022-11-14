use rodio::{OutputStream, Source};
use std::time::Duration;

struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    idx: f32,
    idx_inc: f32,
}

impl WaveTableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WaveTableOscillator {
        return WaveTableOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            idx: 0.0,
            idx_inc: 0.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.idx_inc = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.idx += self.idx_inc;
        self.idx %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_idx = self.idx as usize;
        let next_idx = (truncated_idx + 1) % self.wave_table.len();

        let next_idx_weight = self.idx - truncated_idx as f32;
        let truncated_idx_weight = 1.0 - next_idx_weight;

        return truncated_idx_weight * self.wave_table[truncated_idx]
            + next_idx_weight * self.wave_table[next_idx];
    }
}

impl Iterator for WaveTableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

impl Source for WaveTableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

fn sine(size: usize) -> Vec<f32> {
    let mut wave_table = Vec::with_capacity(size);

    for n in 0..size {
        let phase = 2.0 * std::f32::consts::PI * n as f32 / size as f32;
        wave_table.push((phase).sin());
    }

    return wave_table;
}

fn square(size: usize) -> Vec<f32> {
    let mut wave_table = Vec::with_capacity(size);

    for n in 0..size {
        let phase = 2.0 * std::f32::consts::PI * n as f32 / size as f32;
        wave_table.push(if phase < std::f32::consts::PI { 1 } else { -1 } as f32)
    }

    return wave_table;
}

fn triangle(size: usize) -> Vec<f32> {
    let mut wave_table = Vec::with_capacity(size);

    for n in 0..size {
        let twopi = 2.0 * std::f32::consts::PI;
        let phase = twopi * n as f32 / size as f32;
        let val = 2.0 * (phase * (1.0 / twopi)) - 1.0;

        wave_table.push(if val < 0.0 { -val } else { 2.0 * (val - 0.5) } as f32)
    }

    return wave_table;
}

fn sawtooth(size: usize) -> Vec<f32> {
    let mut wave_table = Vec::with_capacity(size);

    for n in 0..size {
        let twopi = 2.0 * std::f32::consts::PI;
        let phase = twopi * n as f32 / size as f32;

        wave_table.push(2.0 * (phase * (1.0 / twopi)) - 1.0 as f32)
    }

    return wave_table;
}

fn main() {
    let wave_table_size = 64;

    let wave_table = sawtooth(wave_table_size);

    let mut osc = WaveTableOscillator::new(44100, wave_table);
    osc.set_frequency(80.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(osc.convert_samples());

    std::thread::sleep(Duration::from_secs(5))
}
