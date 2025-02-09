use rand::Rng;
use core::panic;
use std::{cmp::Ordering, fmt, sync::{Arc,Mutex}, thread::{self, JoinHandle}};
use num_cpus;

const MAX_MUTATION_CHANCE: u8 = 100; //cant be higher than 100%

/// Represents an individual in the population.
/// Each individual has a set of genes (represented as a vector of booleans),
/// a gene length, and a fitness score.
#[derive(Eq, Debug, Clone)]
pub struct Individual {
    gene_length: usize,
    genes: Vec<bool>,
    fitness: u64,
}

impl Individual {
    /// Randomizes the genes of the individual.
    /// Each gene has a 50% chance of being `true` or `false`.
    fn randomize(&mut self) {
        self.genes = vec![true; self.gene_length];
        for i in 0..self.gene_length {
            let rand = rand::rng().random_range(0..=1);
            if rand == 0 {
                self.genes[i] = false
            }
        }
    }

    pub fn get_genes(&self) -> Vec<bool>{
        return self.genes.clone();
    }

    pub fn set_fitness(&mut self, fitness:u64) {
        self.fitness = fitness;
    }

    /// Mutates the gene at the specified index with a certain probability.
    /// The mutation chance is determined by the `mutation_chance` parameter.
    ///
    /// # Arguments
    /// * `index` - The index of the gene to potentially mutate.
    /// * `mutation_chance` - The probability of mutation (0-100).
    fn mutate_at_index(&mut self, index: usize, mutation_chance: u8) {
        let rand = rand::rng().random_range(0..100);
        if rand < mutation_chance {
            self.genes[index] = !self.genes[index];
        }
    }
}

/// Implements the `PartialOrd` trait for the `Individual` struct.
/// This allows individuals to be compared based on their fitness scores.
impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implements the `PartialEq` trait for the `Individual` struct.
/// This allows individuals to be compared for equality based on their fitness scores.
impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

/// Implements the `Ord` trait for the `Individual` struct.
/// This allows individuals to be ordered based on their fitness scores.
impl Ord for Individual {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fitness.cmp(&other.fitness)
    }
}

/// Implements the `Display` trait for the `Individual` struct.
/// This allows the individual to be printed in a human-readable format,
/// where each gene is represented as '1' (true) or '0' (false).
impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut temp = String::new();
        for i in 0..self.gene_length {
            if self.genes[i] {
                temp.push('1');
            } else {
                temp.push('0');
            }
        }
        write!(f, "Genes: {} Fitness: {}", temp, self.fitness)
    }
}

/// Represents a population of individuals.
/// The population contains a vector of individuals, the size of the population,
/// the number of parents to select for reproduction, and the mutation chance.
#[derive(Debug,Clone)]
pub struct Population {
    pub individuals: Vec<Individual>,
    population_size: usize,
    parent_count: usize,
    mutation_chance: u8,
    multi_threaded:bool
}

impl Population {
    /// Randomizes the genes of all individuals in the population.
    fn randomize_population(&mut self) {
        for i in 0..self.population_size {
            self.individuals[i].randomize();
        }
    }

    /// Creates a new individual (child) from a set of parents.
    /// The child's genes are a combination of the parents' genes, with a chance of mutation.
    ///
    /// # Arguments
    /// * `parents` - A vector of parent individuals used to create the child.
    ///
    /// # Returns
    /// A new `Individual` representing the child.
    fn create_child(&self, parents: Vec<Individual>) -> Individual {
        let mut individual: Individual = Individual {
            gene_length: parents[0].gene_length,
            genes: vec![false; parents[0].gene_length],
            fitness: 0,
        };

        for i in 0..individual.gene_length {
            let rand = rand::rng().random_range(0..parents.len());
            individual.genes[i] = parents[rand].genes[i];
            individual.mutate_at_index(i, self.mutation_chance);
        }

        return individual;
    }

    /// Generates the next generation of individuals based on the current population.
    /// The top-performing individuals are selected as parents, and new individuals
    /// are created through recombination and mutation.
    pub fn next_generation(&mut self) {
        self.individuals.sort_by(|a,b| b.cmp(a));
        self.individuals.truncate(self.parent_count);
        
        // //Single threaded approch
        if !self.multi_threaded{
            let mut next_gen_individuals = vec![];
            for _i in 0..self.population_size - self.individuals.len() {
                next_gen_individuals.push(self.create_child(self.individuals.clone()));
            }
            self.individuals.append(&mut next_gen_individuals);
        }

        else{
            // //Multi threaded approach
            let mut join_handles: Vec<JoinHandle<()>> = vec![];
            let thread_count = if num_cpus::get() >= self.population_size{ self.population_size} else{num_cpus::get()};
            let next_gen_individuals = Arc::new(Mutex::new(vec![]));
            let mut ammount_left = self.population_size - self.individuals.len();
            let chunk_size = self.population_size / thread_count;
            let mutation_chance = self.mutation_chance;
            let parents = Arc::new(self.individuals.clone());
            for i in 0..thread_count{
                let next_gen_individuals = Arc::clone(&next_gen_individuals);
                let mut end = if ammount_left < chunk_size{
                    ammount_left
                } else{
                    chunk_size
                };
                ammount_left -= end;
                if ammount_left > 0 && i+1 == thread_count{
                    end += ammount_left;
                }
                let parents = Arc::clone(&parents);
                join_handles.push(thread::spawn(move || {
                        for _i in 0..end{
                            let mut individual: Individual = Individual {
                                gene_length: parents[0].gene_length,
                                genes: vec![false; parents[0].gene_length],
                                fitness: 0,
                            };
                    
                            for i in 0..individual.gene_length {
                                let rand = rand::rng().random_range(0..parents.len());
                                individual.genes[i] = parents[rand].genes[i];
                                individual.mutate_at_index(i, mutation_chance);
                            }
                            let mut next_gen_individuals = next_gen_individuals.lock().unwrap();
                            next_gen_individuals.push(individual);
                        }
                    })
                )


            }

            // Wait for all threads to finish
            for handle in join_handles{
                handle.join().unwrap();
            }
            let mut next_gen_individuals = Arc::try_unwrap(next_gen_individuals)
                .unwrap()
                .into_inner()
                .unwrap();
            self.individuals.append(&mut next_gen_individuals);
        }
    }

    /// Prints the top `count` individuals in the population, sorted by fitness.
    ///
    /// # Arguments
    /// * `count` - The number of individuals to print.
    pub fn print(&mut self, mut count: usize) {
        self.individuals.sort_by(|a,b| b.cmp(a));
        if count > self.population_size {
            count = self.population_size;
        }
        println!("{}", self);
        for i in 0..count {
            println!("{}", self.individuals[i]);
        }
    }

    pub fn get_population_size(&self) -> usize{
        self.population_size
    }
}

/// Implements the `Display` trait for the `Population` struct.
/// This allows the population to be printed in a human-readable format,
/// showing the population size, parent count, and mutation chance.
impl fmt::Display for Population {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "population_size: {} parent_count: {} mutation_chance: {}",
            self.population_size, self.parent_count, self.mutation_chance
        )
    }
}

/// Initializes a new population with random individuals.
///
/// # Arguments
/// * `gene_length` - The length of the gene sequence for each individual.
/// * `population_size` - The number of individuals in the population.
/// * `parent_count` - The number of parents to select for reproduction.
/// * `mutation_chance` - The chance of mutation for each gene (0-100).
///
/// # Returns
/// A new `Population` with randomized individuals.
pub fn init_population(
    gene_length: usize,
    population_size: usize,
    parent_count: usize,
    mut mutation_chance: u8,
    multi_threaded: bool
) -> Population {
    if mutation_chance > MAX_MUTATION_CHANCE {
        mutation_chance = MAX_MUTATION_CHANCE;
    }
    if gene_length <=0{
        panic!("the gene length cannot be less than 1");
    }
    if parent_count < 1{
        panic!("parent count must be 1 or more")
    }
    if parent_count > population_size{
        panic!("parent count cant be larger than population")
    }
    let mut temp = Population {
        individuals: vec![
            Individual {
                gene_length,
                genes: vec![true; gene_length],
                fitness: 0,
            };
            population_size
        ],
        population_size,
        parent_count,
        mutation_chance,
        multi_threaded
    };
    temp.randomize_population();
    return temp;
}