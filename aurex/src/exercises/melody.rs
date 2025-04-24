use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    midi,
    random::{RandomThings, SeededRandomThings},
    sequence::{Play, Rhythm, Sequence},
    theory::{intervals::Interval, scales},
};

use super::Exercise;

pub struct MelodyExercise<const S: usize, const R: usize> {
    bpm: f64,
    root: wmidi::Note,
    scale: Vec<Interval>,
    steps: SeededRandomThings<isize, S>,
    rhythms: SeededRandomThings<Vec<Rhythm>, R>,
    rest_probability: f64,
    amount_of_beats: f64,
    instrument: wmidi::U7,
    rng: StdRng,
}

pub struct MelodyExerciseSettings<const S: usize, const R: usize> {
    pub bpm: f64,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
    pub steps: RandomThings<isize, S>,
    pub rhythms: RandomThings<Vec<Rhythm>, R>,
    pub rest_probability: f64,
    /// Length of the melody in beats
    pub amount_of_beats: f64,
    pub instrument: wmidi::U7,
    pub seed: u64,
}

impl<const S: usize, const R: usize> MelodyExercise<S, R> {
    pub fn new(
        MelodyExerciseSettings {
            bpm,
            root,
            scale,
            steps,
            rhythms,
            rest_probability,
            amount_of_beats,
            instrument,
            seed,
        }: MelodyExerciseSettings<S, R>,
    ) -> Self {
        Self {
            bpm,
            root,
            scale,
            steps: steps.with_seed(seed),
            rhythms: rhythms.with_seed(seed),
            rest_probability,
            amount_of_beats,
            instrument,
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl<const S: usize, const R: usize> Default for MelodyExerciseSettings<S, R> {
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
            seed: 0,
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

                let note = if self.rng.random::<f64>() < self.rest_probability {
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

        if sequence.length_in_beats() <= 4. {
            sequence.add_to_end(
                Play::Rest.with_duration(Rhythm::Beats(4. - sequence.length_in_beats())),
            );
        } else {
            panic!("TODO: melodies can be at most 4 beats now");
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
