use crate::{
    drums,
    exercises::Exercise,
    metronome::Metronome,
    sequence::{Play, Rhythm, Sequence},
};

pub struct PlayOnce<E: Exercise, M: Metronome> {
    pub exercise: E,
    pub metronome: M,
    pub loops: usize,
}

impl<E: Exercise, M: Metronome> Exercise for PlayOnce<E, M> {
    fn generate(&self) -> Sequence {
        let count_off = drums::count_off(self.bpm());

        let metronome = M::generate(self.bpm()).r#loop(self.loops * 2);
        let mut total_sequence = Sequence::new(self.bpm());

        for _ in 0..self.loops {
            let sequence = self.exercise.generate();
            let sequence_length = sequence.length_in_beats();
            assert_eq!(sequence_length % 4., 0.); // TODO: not a real requirement but convenient during this refactor, remove later
            total_sequence = total_sequence.combine_at_end(sequence);

            let mut empty_sequence = Sequence::new(self.bpm());
            empty_sequence.add_to_end(Play::Rest.with_duration(Rhythm::Beats(sequence_length)));

            total_sequence = total_sequence.combine_at_end(empty_sequence);
        }

        count_off.combine_at_end(total_sequence.combine_simultaneous(metronome))
    }

    fn instrument(&self) -> wmidi::U7 {
        self.exercise.instrument()
    }

    fn bpm(&self) -> u64 {
        self.exercise.bpm()
    }
}
