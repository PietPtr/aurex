use crate::{
    drums, midi,
    sequence::{Play, Rhythm, Sequence},
    theory::{scales, Interval},
};

use super::Exercise;

/// Exercise that plays a three note chord built from one of the notes from the scale.
pub struct DiatonicChordExercise {
    pub bpm: u64,
    pub loops: u32,
    pub root: wmidi::Note,
    pub scale: Vec<Interval>,
}

impl Exercise for DiatonicChordExercise {
    fn play(self) {
        let mut sequence = Sequence::new(self.bpm);
        let scale = scales::scale(self.root, &self.scale);

        for _ in 0..self.loops {
            let chord_root_index = rand::random_range(0..scale.len());
            let third_index = (chord_root_index + 2) % scale.len();
            let fifth_index = (third_index + 2) % scale.len();
            let root = *scale.get(chord_root_index).unwrap();
            let third = *scale.get(third_index).unwrap();
            let fifth = *scale.get(fifth_index).unwrap();

            sequence.add_chord(
                &[Play::Note(root), Play::Note(third), Play::Note(fifth)],
                Rhythm::DoubleWhole,
                wmidi::Channel::Ch1,
            );
        }

        let count_off = drums::count_off(self.bpm);
        let metronome = drums::metronome_emphasis(self.bpm).r#loop(self.loops * 2);
        let sequence = sequence.combine_simultaneous(metronome);

        let mut conn = midi::open_midi_connection("128:0");
        (count_off.combine_at_end(sequence)).play(&mut conn);
    }
}
