use rand::Rng;

use crate::{
    drums, midi,
    random::RandomThings,
    sequence::{Play, Rhythm, Sequence},
    theory::{scales, Interval},
};

use super::Exercise;

/// Generates a melody of at most 3 beats consisting of querter and eighth notes and triplets
/// Never leaping more than 2 steps in the scale
pub struct MelodyExercise<const S: usize, const R: usize> {
    pub bpm: u64,
    pub loops: u32,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
    pub steps: RandomThings<isize, S>,
    pub rhythms: RandomThings<Vec<Rhythm>, R>,
    pub rest_probability: f64,
    /// Length of the melody in beats
    pub amount_of_beats: f64,
}

impl<const S: usize, const R: usize> Default for MelodyExercise<S, R> {
    fn default() -> Self {
        const ARRAY_REPEAT_VALUE: Vec<Rhythm> = Vec::new();
        Self {
            bpm: 80,
            loops: 1,
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
        }
    }
}

impl<const S: usize, const R: usize> Exercise for MelodyExercise<S, R> {
    fn play(self) {
        let mut sequence = Sequence::new(self.bpm);
        let scale = scales::scale(self.root, &self.scale);
        let mut note_index = 0; // always start on root
        for _ in 0..self.loops {
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
            }

            // Fix for weights which bias the melody to go up
            if note_index == scale.len() - 1 {
                note_index = 0;
            }

            sequence.add_to_end(Play::Rest.with_duration(Rhythm::Beats(8. - beats)));
        }

        sequence.add_to_end(Play::Note(self.root).with_duration(Rhythm::Half));

        let count_off = drums::count_off(self.bpm);
        let metronome = drums::metronome_emphasis(self.bpm).r#loop(self.loops * 2);
        let sequence = sequence.combine_simultaneous(metronome);

        let mut conn = midi::open_midi_connection("128:0");
        (count_off.combine_at_end(sequence)).play(&mut conn);
    }
}
