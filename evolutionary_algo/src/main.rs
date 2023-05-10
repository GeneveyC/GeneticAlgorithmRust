use evolutionary_algo::Individual;
use evolutionary_algo::RandomEngine;
use evolutionary_algo::create_population;
use evolutionary_algo::init_random_engine;


fn main() {
    println!("Welcome to the evolutionary algorithm code !");
    let population:Vec<Individual> = create_population( 5 );
    println!("Population size: {:?}", population.len());

    init_random_engine( 13 );
    let value = RandomEngine::rand_int(1, 100);
    println!("Random value: {:?}", value);
}
