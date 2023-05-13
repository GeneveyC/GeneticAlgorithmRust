pub struct Individual {
    pub genes : Vec<u32>,
    pub fitness: f32,
}

impl Individual {
    pub fn new(genes : Vec<u32>) -> Self {
        let fitness = 0.0;
        Individual { genes , fitness }
    }

    pub fn mutate(&mut self) {
        
    }

    pub fn evaluate(&mut self) {

    }

    pub fn to_string(&mut self) {
        println!("Genes : ");
        for gen in self.genes.iter() {
            print!(" {:?}", gen);
        }
        println!();
    }
}