mode parameters::Parameters;
mod individual::Individual;

pub struct Algorithm {
    pub population : Vec<Individual>,
    pub nb_generation : u32,
    pub best_fitness : f64,
}

impl Algorithm {
    pub fn new() -> Self {
        
    }
    
    pub fn run(&self) {

    }

    pub fn selection(&self) {

    }

    pub fn survive(&self) {

    }
}