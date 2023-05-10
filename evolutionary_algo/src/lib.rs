
mod random_engine;
mod individual;

pub use random_engine::RandomEngine;
pub use individual::Individual;

pub fn create_population(nb_population : u32) -> Vec<Individual> {
    let mut individuals:Vec<Individual> = Vec::new();

    for _ in 0..nb_population {
        let mut gen: Vec<u32> = Vec::new();
        gen.push(5);
        gen.push(7);

        let ind = Individual::new(gen);
        individuals.push(ind);
    }
    individuals
}

pub fn init_random_engine(seed : u64) -> RandomEngine {
    let rnd_engine = RandomEngine::new(seed);
    rnd_engine
}