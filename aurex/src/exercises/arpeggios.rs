use crate::{
    exercises::Exercise,
    midi,
    sequence::{Play, Rhythm, Sequence},
    theory::scales,
};

pub struct OneOctaveArpeggios {
    pub root: wmidi::Note,
    pub bpm: f64,
}

impl Exercise for OneOctaveArpeggios {
    fn generate(&mut self) -> crate::sequence::Sequence {
        let scale = scales::scale(self.root, scales::TWO_OCTAVE_MAJOR);

        let mut sequence = Sequence::new(self.bpm);

        for degree in 0..7 {
            let arpeggio = vec![
                scale[degree],
                scale[(degree + 2) % scale.len()],
                scale[(degree + 4) % scale.len()],
                scale[(degree + 6) % scale.len()],
            ];

            for note in arpeggio {
                sequence.add_to_end(Play::Note(note).with_duration(Rhythm::Quarter));
            }
        }

        sequence
    }

    fn instrument(&self) -> wmidi::U7 {
        midi::FINGERED_BASS
    }

    fn bpm(&self) -> f64 {
        self.bpm
    }
}
