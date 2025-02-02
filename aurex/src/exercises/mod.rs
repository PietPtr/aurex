pub mod chords;
pub mod in_scale;
pub mod known_root;
pub mod melody;

pub trait Exercise {
    fn play(self);
}
