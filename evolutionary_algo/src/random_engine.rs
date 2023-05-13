use rand::prelude::*;

pub struct RandomEngine {
    pub seed : u64,
    pub engine: StdRng,
}

impl RandomEngine {
    pub fn new(seed : u64) -> Self {
        let engine = StdRng::seed_from_u64( seed );
        RandomEngine { seed, engine }
    }

    pub fn rand_int(&mut self, min : u32, max : u32) -> u32 {
        self.engine.gen_range(min..max)
    }
}