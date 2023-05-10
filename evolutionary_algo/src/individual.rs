pub struct Individual {
    pub genomes : Vec<u32>,
    pub fitness: f32,
}

impl Individual {
    pub fn new(genomes : Vec<u32>) -> Self {
        let fitness = 0.0;
        Individual { genomes , fitness}
    }
}