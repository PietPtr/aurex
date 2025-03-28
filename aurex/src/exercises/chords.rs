use crate::{
    midi,
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
    fn generate(&self) -> Sequence {
        let mut sequence = Sequence::new(self.bpm);
        let scale = scales::scale(self.root, &self.scale);

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

        sequence
    }

    fn instrument(&self) -> wmidi::U7 {
        midi::GRAND_PIANO
    }

    fn bpm(&self) -> u64 {
        self.bpm
    }
}
