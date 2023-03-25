extern crate rand;
extern crate rand_distr;

use rand::Rng;
use rand_distr::{Distribution, Gamma};

fn main() {
    // Define the shape and scale parameters of the gamma distribution
    let shape = 2.0;
    let scale = 1.0;

    // Create a gamma distribution with the given shape and scale parameters
    let gamma = Gamma::new(shape, scale).unwrap();

    // Create a random number generator and sample values from the gamma distribution
    let mut rng = rand::thread_rng();
    let mut values = Vec::new();

    for _ in 0..1000 {
        let x = gamma.sample(&mut rng);
        values.push(x);
    }

    // Calculate the mean and variance of the sampled values
    let mean = values.iter().sum::<f64>() / (values.len() as f64);
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (values.len() as f64);

    // Print the mean and variance
    println!("Mean: {}", mean);
    println!("Variance: {}", variance);
}
