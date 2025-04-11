use rand::Rng;

use crate::{
    midi,
    random::RandomThings,
    sequence::{Play, Rhythm, Sequence},
    theory::{intervals::Interval, scales},
};

use super::Exercise;

/// Generates a melody of at most 3 beats consisting of querter and eighth notes and triplets
/// Never leaping more than 2 steps in the scale
pub struct MelodyExercise<const S: usize, const R: usize> {
    pub bpm: f64,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
    pub steps: RandomThings<isize, S>,
    pub rhythms: RandomThings<Vec<Rhythm>, R>,
    pub rest_probability: f64,
    /// Length of the melody in beats
    pub amount_of_beats: f64,
    pub instrument: wmidi::U7,
}

impl<const S: usize, const R: usize> Default for MelodyExercise<S, R> {
    fn default() -> Self {
        const ARRAY_REPEAT_VALUE: Vec<Rhythm> = Vec::new();
        Self {
            bpm: 80.,
            root: wmidi::Note::C4,
            scale: scales::MAJOR.to_vec(),
            steps: RandomThings {
                things: [0; S],
                weights: [0; S],
            },
            rhythms: RandomThings {
                things: [ARRAY_REPEAT_VALUE; R],
                weights: [0; R],
            },
            rest_probability: 0.0,
            amount_of_beats: 4.0,
            instrument: midi::FINGERED_BASS,
        }
    }
}

impl<const S: usize, const R: usize> Exercise for MelodyExercise<S, R> {
    fn generate(&mut self) -> Sequence {
        let mut sequence = Sequence::new(self.bpm);
        // TODO: some sort of utility that allows scales to be drawn from indefinitely, maybe as an iterator with 2 directions?
        let scale = scales::scale(self.root, &self.scale);

        let mut note_index = 0; // always start on root
        let mut beats = 0.;

        while beats < self.amount_of_beats {
            let mut rhythms = self.rhythms.sample().clone();

            while let Some(rhythm) = rhythms.pop() {
                let note = scale[note_index];

                let note = if rand::rng().random::<f64>() < self.rest_probability {
                    Play::Rest.with_duration(rhythm.clone())
                } else {
                    Play::Note(note).with_duration(rhythm.clone())
                };

                sequence.add_to_end(note);

                beats += rhythm.beats();

                let new_note_index = (note_index as isize)
                    .saturating_add(*self.steps.sample())
                    .clamp(0, (scale.len() - 1) as isize);
                note_index = new_note_index as usize;
            }

            // Fix for weights which bias the melody to go up
            if note_index == scale.len() - 1 {
                note_index = 0;
            }
        }

        sequence
    }

    fn instrument(&self) -> wmidi::U7 {
        self.instrument
    }

    fn bpm(&self) -> f64 {
        self.bpm
    }
}
