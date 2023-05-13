use evolutionary_algo::*;

fn main() {
    println!("Welcome to the evolutionary algorithm code !");
    
    // Define the parameters of our model
    let mut param = Parameters::default();
    println!("Parameters (rate mutation): {:?}", param.rate_mutation);
    let rng_value = param.rng.rand_int(0, 10);
    println!("Random value: {:?}", rng_value);

    let population:Vec<Individual> = create_population( 5 );
    println!("Population size: {:?}", population.len());

    let mut ind1 = create_individual_from_scratch( 5 );
    let mut ind2 = create_individual_from_scratch( 7 );

    ind1.to_string();
    ind2.to_string();

    let mut ind3 = create_individual_from_two_parents(ind1, ind2);
    ind3.to_string();
}