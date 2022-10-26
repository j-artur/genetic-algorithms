use std::cmp::Ordering;

use crate::types::{Number, Pair};

pub const POPULATION_SIZE: u64 = 10;
pub const MUTATION_RATE: f64 = 0.2;
pub const MAX_GENERATIONS: u64 = 20;

/// The function to be optimized by the genetic algorithm (f(x,y) = sqrt(x^2 + 2*y^4))
pub fn function(x: Number, y: Number) -> f64 {
    f64::sqrt(x.f64().powi(3) + 2.0 * y.f64().powi(4))
}

/// Returns whether a pair is fitter than another pair based on the value of f(x, y) (the lower the better)
pub fn compare(p1: &Pair, p2: &Pair) -> Ordering {
    let z1 = function(p1.x(), p1.y());
    let z2 = function(p2.x(), p2.y());

    if z1 < z2 {
        Ordering::Less
    } else if z1 > z2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Selects the fittest pair from a population
pub fn fittest_pair(pairs: &Vec<Pair>) -> Pair {
    let mut pairs = pairs.clone();
    pairs.sort_by(|a, b| compare(a, b));
    pairs[0]
}
