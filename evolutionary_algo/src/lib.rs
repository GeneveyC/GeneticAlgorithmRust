
mod random_engine;
mod individual;
mod parameters;

pub use random_engine::RandomEngine;
pub use individual::Individual;
pub use parameters::Parameters;


pub fn create_population(nb_population : u32) -> Vec<Individual> {
    let mut individuals:Vec<Individual> = Vec::new();

    for _ in 0..nb_population {
        let gen: Vec<u32> = vec![5, 7];
        let ind = Individual::new(gen);
        individuals.push(ind);
    }
    individuals
}