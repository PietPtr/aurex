use crate::{drums, sequence::Sequence};

use super::Metronome;

pub struct BackbeatMetronome {}

impl Metronome for BackbeatMetronome {
    fn generate(bpm: u64) -> Sequence {
        drums::basic_backbeat(bpm)
    }
}
