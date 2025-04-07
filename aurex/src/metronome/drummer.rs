use crate::{drums, sequence::Sequence};

use super::Metronome;

pub struct BackbeatDrummer {}

impl Metronome for BackbeatDrummer {
    fn generate(bpm: f64) -> Sequence {
        drums::basic_backbeat(bpm)
    }
}

// TODO: sounds shit until velocity is modifiable, currently hardcoded to 50...
pub struct SixteenthsDrummer {}

impl Metronome for SixteenthsDrummer {
    fn generate(bpm: f64) -> Sequence {
        drums::sixteenths_drums(bpm)
    }
}

pub struct TwelveEighthDrummer {}

impl Metronome for TwelveEighthDrummer {
    fn generate(bpm: f64) -> Sequence {
        drums::twelveeight_drums(bpm)
    }
}
