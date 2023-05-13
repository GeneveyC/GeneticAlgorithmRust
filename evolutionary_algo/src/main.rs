use evolutionary_algo::*;
//use evolutionary_algo::init_random_engine;


fn main() {
    println!("Welcome to the evolutionary algorithm code !");
    let population:Vec<Individual> = create_population( 5 );
    println!("Population size: {:?}", population.len());

    let mut random_engine = RandomEngine::new( 13 );
    let value  = random_engine.rand_int(0, 100);
    println!("Random value: {:?}", value);

    let mut ind1 = create_individual_from_scratch( 5 );
    let mut ind2 = create_individual_from_scratch( 7 );

    ind1.to_string();
    ind2.to_string();

    let mut ind3 = create_individual_from_two_parents(ind1, ind2);
    ind3.to_string();
}