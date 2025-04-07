use crate::{
    drums::{self, METRONOME_TICK},
    sequence::{Play, Rhythm, Sequence},
};

use super::Metronome;

pub struct TickEveryBeatMetronome {}

impl Metronome for TickEveryBeatMetronome {
    fn generate(bpm: f64) -> Sequence {
        drums::metronome(bpm)
    }
}

pub struct EmphasisOneMetronome {}

impl Metronome for EmphasisOneMetronome {
    fn generate(bpm: f64) -> Sequence {
        drums::metronome_emphasis(bpm)
    }
}

pub struct TwoAndFourMetronome {}

impl Metronome for TwoAndFourMetronome {
    fn generate(bpm: f64) -> Sequence {
        drums::metronome_backbeat(bpm)
    }
}

pub struct OffbeatMetronome {}

impl Metronome for OffbeatMetronome {
    fn generate(bpm: f64) -> Sequence {
        let mut sequence = Sequence::new(bpm);

        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Eighth));

        for _ in 0..3 {
            sequence.add_to_end(METRONOME_TICK.with_duration(Rhythm::Quarter));
        }

        sequence.add_to_end(METRONOME_TICK.with_duration(Rhythm::Eighth));

        sequence
    }
}
