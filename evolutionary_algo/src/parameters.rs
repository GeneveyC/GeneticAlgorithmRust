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


pub struct Parameters {
    pub nb_individual : u32,
    pub nb_genes : u32,
    pub nb_max_iterations : u32,
    pub min_fitness : f64,
    pub rate_mutation : f64,
    pub rate_add_genes : f64,
    pub rate_del_genes : f64,
    pub rate_crossover : f64,
    pub rng : RandomEngine,
}

impl Default for Parameters {
    fn default() -> Self {
        let nb_individual = 20;
        let nb_genes = 10;
        let nb_max_iterations = 50;
        let min_fitness = 0.0;
        let rate_mutation = 0.1;
        let rate_add_genes = 0.2;
        let rate_del_genes = 0.1;
        let rate_crossover = 0.6;
        let rng = RandomEngine::new(0);

        Parameters { nb_individual, nb_genes, nb_max_iterations, min_fitness, rate_mutation, rate_add_genes, rate_del_genes, rate_crossover, rng}
    }
}