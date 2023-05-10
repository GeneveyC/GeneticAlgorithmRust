use evolutionary_algo::Individual;
use evolutionary_algo::create_population;

fn main() {
    println!("Welcome to the evolutionary algorithm code !");
    let population:Vec<Individual> = create_population( 5 );
    println!("Population size: {:?}", population.len());
}
