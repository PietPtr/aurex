use rand::distr::{weighted::WeightedIndex, Distribution};

pub struct RandomThings<T, const S: usize> {
    // TODO: pub weighted_things: [(u32, T); S]
    pub things: [T; S],
    pub weights: [u32; S],
}

impl<T, const S: usize> RandomThings<T, S> {
    pub fn sample(&self) -> &T {
        let dist = WeightedIndex::new(&self.weights).unwrap();

        let sample = self.things.get(dist.sample(&mut rand::rng())).unwrap();

        sample
    }
}
