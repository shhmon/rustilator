use rodio::Source;
use std::time::Duration;
pub struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    idx: f32,
    idx_inc: f32,
}

impl WaveTableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> WaveTableOscillator {
        return WaveTableOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            idx: 0.0,
            idx_inc: 0.0,
        };
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.idx_inc = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    pub fn get_sample(&mut self) -> f32 {
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
