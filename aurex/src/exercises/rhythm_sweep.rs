use crate::{
    midi,
    sequence::{Play, Rhythm, Sequence},
};

use super::Exercise;

/// Given a subdivision length, sweep it along the bar,
/// playing each place for the subdivision a given amount of times.
/// Use RhythmSweepExerciseDef.into() to obtain RhythmSweepExercise
pub struct RhythmSweepExercise {
    bpm: f64,
    note: wmidi::Note,
    subdivision_length: Rhythm,
    sweep_step_length: Rhythm,
    repeats_per_place: usize,
    runs: usize,
}

pub struct RhythmSweepExerciseDef {
    pub bpm: f64,
    pub note: wmidi::Note,
    pub subdivision_length: Rhythm,
    pub sweep_step_length: Rhythm,
    pub repeats_per_place: usize,
}

impl From<RhythmSweepExerciseDef> for RhythmSweepExercise {
    fn from(
        RhythmSweepExerciseDef {
            bpm,
            note,
            subdivision_length,
            repeats_per_place,
            sweep_step_length,
        }: RhythmSweepExerciseDef,
    ) -> Self {
        Self {
            bpm,
            note,
            subdivision_length,
            repeats_per_place,
            runs: 0,
            sweep_step_length,
        }
    }
}

impl Exercise for RhythmSweepExercise {
    fn generate(&mut self) -> Sequence {
        let mut sequence = Sequence::new(self.bpm);

        // Zero-indexed subdivision index
        let subdivision = self.runs / self.repeats_per_place;
        let beats_of_rest_before_subdivision = subdivision as f64 * self.sweep_step_length.beats();
        let beats_of_rest_after_subdivision =
            4.0 - beats_of_rest_before_subdivision - self.subdivision_length.beats();

        // Done sweeping
        if beats_of_rest_after_subdivision < 0. {
            return sequence;
        }

        sequence
            .add_to_end(Play::Rest.with_duration(Rhythm::Beats(beats_of_rest_before_subdivision)));

        sequence.add_to_end(Play::Note(self.note).with_duration(self.subdivision_length.clone()));

        sequence
            .add_to_end(Play::Rest.with_duration(Rhythm::Beats(beats_of_rest_after_subdivision)));

        self.runs += 1;
        sequence
    }

    fn instrument(&self) -> wmidi::U7 {
        midi::FINGERED_BASS
    }

    fn bpm(&self) -> f64 {
        self.bpm
    }
}
