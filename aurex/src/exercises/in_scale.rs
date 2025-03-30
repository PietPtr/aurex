use crate::{
    midi,
    sequence::{Play, Rhythm, Sequence},
    theory::{scales, Interval},
};

use super::Exercise;

pub struct InScaleExercise {
    pub bpm: f64,
    pub loops: u32,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
}

impl Exercise for InScaleExercise {
    fn generate(&self) -> Sequence {
        let scale = scales::scale(self.root, scales::MAJOR_PENTATONIC);
        let mut sequence = Sequence::new(self.bpm);
        sequence.add_to_end(Play::RandomNote(scale.clone()).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::RandomNote(scale).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Half));
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Whole));

        sequence
    }

    fn instrument(&self) -> wmidi::U7 {
        todo!()
    }

    fn bpm(&self) -> f64 {
        todo!()
    }
}

pub struct InScaleWithRangeExercise {
    pub bpm: f64,
    pub loops: u32,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
    pub range_start: wmidi::Note,
    pub range_end: wmidi::Note,
}

impl Exercise for InScaleWithRangeExercise {
    fn generate(&self) -> Sequence {
        let notes = scales::scale_range(self.root, &self.scale, self.range_start, self.range_end);

        let mut sequence = Sequence::new(self.bpm);
        sequence.add_to_end(Play::RandomNote(notes.clone()).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::RandomNote(notes).with_duration(Rhythm::Quarter));
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Half));
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Whole));

        sequence
    }

    fn instrument(&self) -> wmidi::U7 {
        midi::FINGERED_BASS
    }

    fn bpm(&self) -> f64 {
        self.bpm
    }
}
