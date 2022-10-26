use crate::config::*;
use crate::types::*;
use crate::util::*;

use rand::Rng;

pub mod config;
pub mod types;
pub mod util;

/// Calculates the fitness of a pair of numbers.
fn fitness(pair: &Pair) -> f64 {
    let f = function(pair.x(), pair.y());
    // Can't divide by zero
    if f == 0.0 {
        f64::MAX
    } else {
        1.0 / f
    }
}

/// Returns the fittest pairs from a population
fn select_parents(pairs: &Vec<Pair>) -> Vec<Pair> {
    // Maps each pair to a tuple containing itself and its fitness
    let fitnesses: Vec<_> = pairs.iter().map(|p| (p, fitness(p))).collect();

    // Maps each pair to a tuple containing itself and its accumulated fitness
    // (the sum of all previous fitnesses + its own)
    let mut acc_fitnesses: Vec<_> = fitnesses
        .iter()
        .scan(0.0, |state, (p, f)| {
            *state += f;
            Some((p, *state))
        })
        .collect();

    let mut parents = Vec::new();

    // Selects the parents by randomly selecting a number between 0 and the highest accumulated fitness
    // and then finding the first pair where the accumulated fitness is greater than it
    for _ in 0..2 {
        // Calculates the highest accumulated fitness
        let highest_fitness = acc_fitnesses.last().unwrap().1;

        let mut rng = rand::thread_rng();
        let chosen_f64 = rng.gen_range(0.0..highest_fitness);

        // Finds the first pair where the accumulated fitness is greater than the chosen number
        let chosen_i = acc_fitnesses
            .iter()
            .enumerate()
            .find_map(|(i, (_, f))| if chosen_f64 < *f { Some(i) } else { None })
            .unwrap();

        // Removes the chosen pair from the list of accumulated fitnesses so that it can't be chosen again
        let chosen_pair = acc_fitnesses.remove(chosen_i).0;

        // And adds it to the list of parents
        parents.push(**chosen_pair);
    }

    parents
}

/// Performs crossover on every pair of individuals in a population
fn crossover(parents: &Vec<Pair>) -> Vec<Pair> {
    let p1 = parents[0];
    let p2 = parents[1];

    // Generates children from the parents
    let children = p1.crossover(&p2);

    children
}

/// Performs mutations on a population based on the mutation rate
fn mutate(generation: &mut Vec<Pair>, mut_rate: f64) {
    let mut rng = rand::thread_rng();
    // For each pair in the population
    generation.iter_mut().for_each(|p| {
        // Mutates the pair based on the mutation rate
        if rng.gen_bool(mut_rate) {
            p.mutate();
        }
    });
}

/// Performs a single generation of the genetic algorithm
fn next_generation(old_gen: &Vec<Pair>, mut_rate: f64) -> (Vec<Pair>, Vec<Pair>) {
    // Selects the parents from the old generation
    let selected_parents = select_parents(&old_gen);

    // Performs crossover on the parents
    let mut new_population = crossover(&selected_parents);

    // Performs mutations on the new population
    mutate(&mut new_population, mut_rate);

    (selected_parents, new_population)
}

fn main() {
    // Number of the current generation
    let mut i = 0;

    // Populates the first generation by randomly generating pairs
    let first_gen: Vec<_> = (0..POPULATION_SIZE).map(|_| Pair::random()).collect();
    print_info(0, &vec![], &first_gen);

    // If the first generation is already the fittest, we're done
    let fittest = fittest_pair(&first_gen);
    if function(fittest.x(), fittest.y()) == 0.0 {
        return;
    }

    // Otherwise, we keep generating new generations until we reach the maximum number of generations
    // or we find the fittest pair (which is the pair with the lowest value of the function)
    let mut old_gen = first_gen;
    loop {
        i += 1;

        // Generates the next generation from the old one
        let (parents, new_gen) = next_generation(&old_gen, MUTATION_RATE);
        print_info(i, &parents, &new_gen);

        // If the new generation is the fittest, we're done
        let fittest = fittest_pair(&new_gen);
        if function(fittest.x(), fittest.y()) == 0.0 || i == MAX_GENERATIONS {
            break;
        } else {
            old_gen = new_gen;
        }
    }
}
