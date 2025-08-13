use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct RandomStream {
    rng: StdRng,
}

impl RandomStream {
    pub fn new(seed: u64) -> Self {
        RandomStream {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn next_f64(&mut self) -> f64 {
        self.rng.r#gen::<f64>()
    }

    pub fn gen_range<T: rand::distributions::uniform::SampleUniform + std::cmp::PartialOrd>(&mut self, range: std::ops::RangeInclusive<T>) -> T {
        self.rng.gen_range(range)
    }
}