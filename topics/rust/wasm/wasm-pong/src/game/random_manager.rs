use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;


pub struct RandomManager {
    generator: SmallRng
}

impl RandomManager {
    pub fn new(seed: u64) -> RandomManager {
        RandomManager {
            generator: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn next_in_range(&mut self, min: f32, max: f32) -> f32 {
        self.generator.gen_range(min, max)
    }
}
