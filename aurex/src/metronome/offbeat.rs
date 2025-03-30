use crate::{
    drums::METRONOME_TICK,
    sequence::{Play, Rhythm, Sequence},
};

use super::Metronome;

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
