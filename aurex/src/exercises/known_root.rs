use crate::{
    drums,
    midi::{self, FINGERED_BASS},
    sequence::{Play, Rhythm, Sequence},
    theory::{scales, Interval},
};

use super::Exercise;

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
        sequence.add_to_end(Play::Rest.with_duration(Rhythm::Whole));

        // TODO: use closure note to repeat last bar instead of silence to easily check if correct?

        let sequence = sequence.r#loop(self.loops).combine_simultaneous(metronome);

        (count_off.combine_at_end(sequence)).play(&mut conn);
    }
}
