use crate::{
    drums, midi,
    sequence::{Play, Rhythm, Sequence},
    theory::{scales, Interval},
};

use super::Exercise;

pub struct InScaleExercise {
    pub bpm: u64,
    pub loops: u32,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
}

impl Exercise for InScaleExercise {
    fn play(self) {
        let mut conn = midi::open_midi_connection("128:0");

        let count_off = drums::count_off(self.bpm);
        let metronome = drums::metronome_emphasis(self.bpm).r#loop(self.loops);
        let scale = scales::scale(self.root, scales::MAJOR_PENTATONIC);

        let mut sequence = Sequence::new(self.bpm);
        sequence.add_to_end(Play::RandomNote(scale.clone()).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::RandomNote(scale).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Half));
        let sequence = sequence.r#loop(self.loops).combine_simultaneous(metronome);

        (count_off.combine_at_end(sequence)).play(&mut conn);
    }
}

pub struct InScaleWithRangeExercise {
    pub bpm: u64,
    pub loops: u32,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
    pub range_start: wmidi::Note,
    pub range_end: wmidi::Note,
}

impl Exercise for InScaleWithRangeExercise {
    fn play(self) {
        let notes = scales::scale_range(self.root, &self.scale, self.range_start, self.range_end);

        let mut conn = midi::open_midi_connection("128:0");

        let count_off = drums::count_off(self.bpm);
        let metronome = drums::metronome_emphasis(self.bpm).r#loop(self.loops);

        let mut sequence = Sequence::new(self.bpm);
        sequence.add_to_end(Play::RandomNote(notes.clone()).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::RandomNote(notes).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Half));
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Whole));

        let sequence = sequence
            .r#loop(self.loops * 2)
            .combine_simultaneous(metronome);
        (count_off.combine_at_end(sequence)).play(&mut conn);
    }
}
