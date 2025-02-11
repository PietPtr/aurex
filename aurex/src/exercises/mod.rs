pub mod chords;
pub mod in_scale;
pub mod known_root;
pub mod melody;
pub mod variator;
pub mod walking;

pub trait Exercise {
    fn play(self);
}
