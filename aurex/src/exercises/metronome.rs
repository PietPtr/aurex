use crate::{drums, metronome::Metronome, midi, sequence::Sequence};

pub struct MetronomeExercise<M: Metronome> {
    pub metronome: M,
    pub bpm: f64,
    pub loops: usize,
    pub countoff: bool,
}

// TODO: if no qsynth, start it

impl<M: Metronome> super::Exercise for MetronomeExercise<M> {
    fn generate(&mut self) -> Sequence {
        let count = if self.countoff {
            drums::count_off(self.bpm)
        } else {
            Sequence::new(self.bpm)
        };

        count.combine_at_end(M::generate(self.bpm).r#loop(self.loops))
    }

    fn instrument(&self) -> wmidi::U7 {
        // TODO: ..channels?
        midi::FINGERED_BASS
    }

    fn bpm(&self) -> f64 {
        self.bpm
    }
}
