use crate::sequence::Sequence;

pub mod backbeat;
pub mod beat;
pub mod emphasis_one;
pub mod offbeat;
pub mod two_and_four;

pub trait Metronome {
    fn generate(bpm: f64) -> Sequence;
}
