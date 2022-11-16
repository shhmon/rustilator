use std::collections::HashMap;

use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::{OutputStream, Source};

mod freqs;
mod osc;
mod waves;

fn main() {
    let wave_table_size = 64;

    let wave_table = waves::sine(wave_table_size);

    let mut osc = osc::WaveTableOscillator::new(44100, wave_table);
    osc.set_frequency(freqs::C[4]);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(osc.convert_samples());

    let device_state = DeviceState::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let o = 5;

        for key in keys.iter() {
            let freq = match key {
                Keycode::A => freqs::C[o],
                Keycode::W => freqs::CS[o],
                Keycode::S => freqs::C[o],
                Keycode::D => freqs::DS[o],
                Keycode::F => freqs::E[o],
                Keycode::T => freqs::F[o],
                Keycode::G => freqs::FS[o],
                Keycode::Y => freqs::G[o],
                Keycode::H => freqs::GS[o],
                Keycode::U => freqs::A[o],
                Keycode::J => freqs::AS[o],
                Keycode::K => freqs::B[o],
                _ => 440.0,
            };

            osc.set_frequency(freq);
        }
    }

    //std::thread::sleep(Duration::from_secs(5))
}
