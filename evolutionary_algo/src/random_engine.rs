use rand::{Rng, rngs::StdRng};

pub struct RandomEngine {
    pub seed : u32,
    pub engine : StdRng,
}

impl RandomEngine {
    pub fn new(u32 : seed) -> Self {
        let mut engine = StdRng::seed_from_u32(seed);
        RandomEngine { seed, engine }
    }

    pub fn randInt(min : u32, max : u32) -> u32 {
        let value = engine.gen_range(min, max);
        value
    }
}