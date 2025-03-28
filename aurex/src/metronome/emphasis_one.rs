use crate::{drums, sequence::Sequence};

use super::Metronome;

pub struct EmphasisOneMetronome {}

impl Metronome for EmphasisOneMetronome {
    fn generate(bpm: u64) -> Sequence {
        drums::metronome_emphasis(bpm)
    }
}
