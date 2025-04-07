use crate::sequence::Sequence;

pub mod drummer;
pub mod metronomes;

pub trait Metronome {
    fn generate(bpm: f64) -> Sequence;
}
