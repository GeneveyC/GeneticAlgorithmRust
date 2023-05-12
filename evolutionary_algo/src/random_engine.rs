use rand::{Rng, StdRng, SeedableRng};

pub struct RandomEngine {
    pub seed : u64,
    pub engine : StdRng,
}

impl RandomEngine {
    pub fn new(seed : u64) -> Self {
        let engine = SeedableRng::seed_from_u64(seed);
        RandomEngine { seed, engine }
    }

    pub fn rand_int(min : u32, max : u32) -> u32 {
        rand::thread_rng().gen_range(min, max)
    }
}