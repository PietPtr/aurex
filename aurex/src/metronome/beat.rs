use crate::{drums, sequence::Sequence};

use super::Metronome;

pub struct BeatMetronome {}

impl Metronome for BeatMetronome {
    fn generate(bpm: f64) -> Sequence {
        drums::metronome(bpm)
    }
}
