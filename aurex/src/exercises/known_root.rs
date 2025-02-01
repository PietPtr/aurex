use crate::{
    drums, midi,
    sequence::{Play, Rhythm, Sequence},
    theory::{scales, Interval},
};

use super::Exercise;

pub struct KnownRootExercise {
    pub bpm: u64,
    pub loops: u32,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
}

impl Exercise for KnownRootExercise {
    fn play(self) {
        let mut conn = midi::open_midi_connection("128:0");

        let count_off = drums::count_off(self.bpm);
        let metronome = drums::metronome_emphasis(self.bpm).r#loop(self.loops);
        let scale = scales::scale(self.root, scales::MAJOR_PENTATONIC);

        let mut sequence = Sequence::new(self.bpm);
        sequence.add_to_end(Play::Note(self.root).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::RandomNote(scale).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Half));
        let sequence = sequence.r#loop(self.loops).combine_simultaneous(metronome);

        (count_off.combine_at_end(sequence)).play(&mut conn);
    }
}
