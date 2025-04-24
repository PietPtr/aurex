use rand::{
    distr::{weighted::WeightedIndex, Distribution},
    rngs::{StdRng, ThreadRng},
    SeedableRng,
};

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

    pub fn with_seed(self, seed: u64) -> SeededRandomThings<T, S> {
        SeededRandomThings::new(seed, self)
    }
}

pub struct SeededRandomThings<T, const S: usize> {
    pub things: [T; S],
    pub weights: [u32; S],
    rng: StdRng,
}

impl<T, const S: usize> SeededRandomThings<T, S> {
    pub fn new(seed: u64, random_things: RandomThings<T, S>) -> Self {
        Self {
            things: random_things.things,
            weights: random_things.weights,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn sample(&mut self) -> &T {
        let dist = WeightedIndex::new(&self.weights).unwrap();

        let sample = self.things.get(dist.sample(&mut self.rng)).unwrap();

        sample
    }
}
