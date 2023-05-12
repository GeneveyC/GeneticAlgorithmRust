
mod random_engine;
mod individual;

pub use random_engine::RandomEngine;
pub use individual::Individual;

pub fn create_population(nb_population : u32) -> Vec<Individual> {
    let mut individuals:Vec<Individual> = Vec::new();

    for _ in 0..nb_population {
        let gen: Vec<u32> = vec![5, 7];
        let ind = Individual::new(gen);
        individuals.push(ind);
    }
    individuals
}

pub fn init_random_engine(seed : u64) -> RandomEngine {
    RandomEngine::new(seed)
}