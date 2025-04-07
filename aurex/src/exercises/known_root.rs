use rand::seq::IndexedRandom;

use crate::{
    midi,
    sequence::{Play, Rhythm, Sequence},
    theory::{intervals::Interval, scales},
};

use super::Exercise;

// TODO: refactor the exercise trait, it should expose a method that generates one unit of exercise, then define several ways to play an exercise (e.g. one that repeats everything twice and one that leaves room)
// (likely means the death of RandomNote which wasn't so useful anyway)
// Could have instruments defined there as well?
// Ability to seed exercises / run with random seed (but its printed)
// new melody exercise where each bar is a slight variation of the previous (one note or rhythm), always starts as an arpeggio of a given chord

// TODO: friendlier randomness, e.g. draw from all options until exhausted, then repeat

// TODO: bad names
pub enum Direction {
    Ascending,
    Descending,
    // Picks random option
    // Both,
}

pub enum RootPosition {
    StartOnRoot,
    EndOnRoot,
    // Picks random option
    // Both,
}

pub struct KnownRootExercise {
    pub bpm: f64,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
    pub direction: Direction,
    pub root_position: RootPosition,
}

impl Exercise for KnownRootExercise {
    fn generate(&mut self) -> Sequence {
        let scale = match self.direction {
            Direction::Ascending => scales::scale(self.root, &self.scale),
            Direction::Descending => scales::descending_scale(self.root, &self.scale),
        };

        let random_note = || scale.choose(&mut rand::rng()).copied().unwrap();

        let mut sequence = Sequence::new(self.bpm);
        match self.root_position {
            RootPosition::StartOnRoot => {
                sequence.add_to_end(Play::Note(self.root).with_duration(Rhythm::Quarter));
                sequence.add_to_end(Play::Note(random_note()).with_duration(Rhythm::Quarter));
            }
            RootPosition::EndOnRoot => {
                sequence.add_to_end(Play::Note(random_note()).with_duration(Rhythm::Quarter));
                sequence.add_to_end(Play::Note(self.root).with_duration(Rhythm::Quarter));
            }
        }

        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Half));

        sequence
    }

    fn instrument(&self) -> wmidi::U7 {
        midi::FINGERED_BASS
    }

    fn bpm(&self) -> f64 {
        self.bpm
    }
}
