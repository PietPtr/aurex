use crate::{drums, sequence::Sequence};

use super::Metronome;

pub struct TwoAndFourMetronome {}

impl Metronome for TwoAndFourMetronome {
    fn generate(bpm: u64) -> Sequence {
        drums::metronome_backbeat(bpm)
    }
}
