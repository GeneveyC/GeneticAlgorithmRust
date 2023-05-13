mod individual;
mod parameters;

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

pub fn create_individual_from_scratch(nb_genes : u32) -> Individual {
    let mut gen : Vec<u32> = Vec::new();

    for i in 0..nb_genes {
        gen.push(i);
    }

    Individual::new(gen)
}


pub fn create_individual_from_one_parents(parent1 : Individual) -> Individual {
    let gen : Vec<u32> = parent1.genes;
    Individual::new(gen)
}

pub fn create_individual_from_two_parents(parent1 : Individual, parent2 : Individual) -> Individual {
    let gen_parent1 : Vec<u32> = parent1.genes;
    let gen_parent2 : Vec<u32> = parent2.genes;

    let mut gen : Vec<u32> = Vec::new();
    for gen1 in gen_parent1.iter() {
        gen.push(*gen1);
    }

    for gen2 in gen_parent2.iter() {
        gen.push(*gen2);
    }

    Individual::new(gen)
}