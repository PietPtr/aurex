use crate::{drums, exercises::Exercise, metronome::Metronome, sequence::Sequence};

/// Assumes a sequence can be stacked indefinitely and plays every generated sequence twice.
pub struct PlayTwice<E: Exercise, M: Metronome> {
    pub exercise: E,
    pub metronome: M,
    pub loops: usize,
}

impl<E: Exercise, M: Metronome> Exercise for PlayTwice<E, M> {
    fn generate(&self) -> Sequence {
        let count_off = drums::count_off(self.bpm());

        let metronome = M::generate(self.bpm()).r#loop(self.loops * 2);
        let mut total_sequence = Sequence::new(self.bpm());

        for _ in 0..self.loops {
            let sequence = self.exercise.generate();
            total_sequence = total_sequence.combine_at_end(sequence.clone());
            total_sequence = total_sequence.combine_at_end(sequence);
        }

        count_off.combine_at_end(total_sequence.combine_simultaneous(metronome))
    }

    fn instrument(&self) -> wmidi::U7 {
        self.exercise.instrument()
    }

    fn bpm(&self) -> f64 {
        self.exercise.bpm()
    }
}
