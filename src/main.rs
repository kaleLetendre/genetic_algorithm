use std::time::Instant;

use genetic_algorithm::Population;
/*
The goal of this program is to
    - spawn a population of individuals
    - score individuals on a task based on their genetics
    - repopulate new generation of idividuals based on best candidates from last generation
    - mutate some individuals to avoid convergence
*/
mod genetic_algorithm;
fn main() {
    let mut now = start_timer("init_population");
    let mut population = genetic_algorithm::init_population(10,100,2,10,true);
    end_timer(now);

    now = start_timer("timing started\n");
    population = alternating_pattern_test(population.clone());
    population.print(10);

    for _i in 0..10{
        
        population.next_generation();
        population = alternating_pattern_test(population.clone());
        population.print(10);
    }
    end_timer(now);


}

fn start_timer(print: &str) -> Instant{
    println!("{}",print);
    Instant::now()
}

fn end_timer(now:Instant){
    println!("Elapsed: {:.2?}", now.elapsed());
}

fn ones_test(mut population:Population) -> Population{
    for i in 0..population.get_population_size(){
        let mut fitness = 0;
        let genes = population.individuals[i].get_genes();
        for gene in genes{
            if gene{fitness = fitness + 1;}
        }
        population.individuals[i].set_fitness(fitness);
    }
    return population;
}

fn alternating_pattern_test(mut population: Population) -> Population {
    for i in 0..population.individuals.len() {
        let mut fitness = 0;
        let genes = population.individuals[i].get_genes();
        for i in 1..genes.len() {
            if genes[i] != genes[i - 1] {
                fitness += 1;
            }
        }
        population.individuals[i].set_fitness(fitness);
    }
    return population;
}

