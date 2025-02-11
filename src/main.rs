use genetic_algorithm::Population;
/*
The script shows the usage of the genetic algorithm
*/
mod genetic_algorithm;

fn main(){
    string_match_example("hello_world");
}
fn string_match_example(string:&str){
    // goal fitness used to break upon reaching goal, 0 to run forever
    let goal_fitness = 12*255; //the fitness calculation is the sum of the 255 minus the difference between the real and guessed char. that 12*255 represent perfect fitness
    let gene_length = string.len() * 8; //I chose this value because each ascii char is 8 bits (1 byte)
    
    //create the population
    let mut population = genetic_algorithm::init_population(gene_length,10,4,15,true,genetic_algorithm::CrossoverType::Byte);

    let mut count = 0;
    loop{
        count+=1;
        // Fitness funtion
        population = string_match_fitness(population.clone(),string);

        //outputs state
        let fittest = population.read_fittest();
        
        if count%1000 == 0{
            println!("| {:?} |\n| {:?} | |fitness: {}| generation: {}|",string.as_bytes(),fittest.get_genes_as_decimal_bytes(),fittest.get_fitness(),count);
        }
        //break when we reach the goal fitness, otherwise you can break with Ctrl+c
        if fittest.get_fitness() >= goal_fitness && goal_fitness != 0{
            println!("you may not like it but this is what peak performance looks like > {:?}",fittest.get_genes_as_decimal_bytes());
            break;
        }

        // crossover the best
        population.next_generation();
        

        
    }


}


fn string_match_fitness(mut population: Population, string:&str) -> Population {
    let bytes = string.as_bytes();
    for i in 0..population.individuals.len(){
        let gene_bytes: Vec<u8> = population.individuals[i].get_genes_as_decimal_bytes();
        let mut fitness = 0;
        for j in 0..string.len(){
            let diff = (gene_bytes[j] as i64-bytes[j] as i64).abs();
            fitness += 255 - diff;  
        }
        population.individuals[i].set_fitness(fitness as u64)
    }
    return population;
}

