use std::rc::Rc;

use crate::{
    drums,
    midi::{self, FINGERED_BASS},
    sequence::{Play, Rhythm, Sequence},
    theory::{scales, Interval},
};

use super::Exercise;

// TODO: refactor the exercise trait, it should expose a method that generates one unit of exercise, then define several ways to play an exercise (e.g. one that repeats everything twice and one that leaves room)
// (likely means the death of RandomNote which wasn't so useful anyway)
// Could have instruments defined there as well?
// Ability to seed exercises / run with random seed (but its printed)
// new melody exercise where each bar is a slight variation of the previous (one note or rhythm), always starts as an arpeggio of a given chord

// TODO: bad names
pub enum Direction {
    Ascending,
    Descending,
}

pub enum RootPosition {
    StartOnRoot,
    EndOnRoot,
}

pub struct KnownRootExercise {
    pub bpm: u64,
    pub loops: u32,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
    pub direction: Direction,
    pub root_position: RootPosition,
}

impl Exercise for KnownRootExercise {
    fn play(self) {
        let mut conn = midi::open_midi_connection("128:0");
        midi::set_instrument(&mut conn, wmidi::Channel::Ch1, FINGERED_BASS);

        let count_off = drums::count_off(self.bpm);
        let metronome = drums::metronome_emphasis(self.bpm).r#loop(self.loops * 2);
        let scale = match self.direction {
            Direction::Ascending => scales::scale(self.root, &self.scale),
            Direction::Descending => scales::descending_scale(self.root, &self.scale),
        };

        // TODO: Make a spell function for scales?
        println!("Known roots exercise on notes:\n{:?}", scale);

        let mut sequence = Sequence::new(self.bpm);
        match self.root_position {
            RootPosition::StartOnRoot => {
                sequence.add_to_end(Play::Note(self.root).with_duration(Rhythm::Quarter));
                sequence.add_to_end(Play::RandomNote(scale).with_duration(Rhythm::Quarter));
            }
            RootPosition::EndOnRoot => {
                sequence.add_to_end(Play::RandomNote(scale).with_duration(Rhythm::Quarter));
                sequence.add_to_end(Play::Note(self.root).with_duration(Rhythm::Quarter));
            }
        }
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Half));

        // Repeat the same notes to check correctness
        sequence.add_to_end(
            Play::ClosureNote(Rc::new(|notes| notes.get(notes.len() - 2).copied()))
                .with_duration(Rhythm::Quarter),
        );
        sequence.add_to_end(
            Play::ClosureNote(Rc::new(|notes| notes.get(notes.len() - 2).copied()))
                .with_duration(Rhythm::Quarter),
        );
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Half));

        // TODO: use closure note to repeat last bar instead of silence to easily check if correct?

        let sequence = sequence.r#loop(self.loops).combine_simultaneous(metronome);

        (count_off.combine_at_end(sequence)).play(&mut conn);
    }
}
