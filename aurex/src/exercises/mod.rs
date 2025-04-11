use crate::{midi, sequence::Sequence};

pub mod chords;
pub mod in_scale;
pub mod known_root;
pub mod melody;
pub mod metronome;
pub mod rhythm_sweep;
pub mod variator;
pub mod walking;

pub trait Exercise {
    fn generate(&mut self) -> Sequence;
    fn instrument(&self) -> wmidi::U7;
    fn bpm(&self) -> f64;
}

pub fn play<E: Exercise>(mut exercise: E) {
    let mut seq = exercise.generate();

    let mut conn = midi::open_midi_connection("FLUID Synth");
    midi::set_instrument(&mut conn, wmidi::Channel::Ch1, exercise.instrument());

    seq.play(&mut conn);
}
