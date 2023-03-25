extern crate rand;

use rand::distributions::{Distribution, Normal};
use rand::thread_rng;


fn main() {
    // Define the mean and standard deviation of the normal distribution
    let mean = 0.0;
    let std_dev = 1.0;

    // Create a normal distribution with the given mean and standard deviation
    let normal = Normal::new(mean, std_dev).unwrap();

    // Create a random number generator and sample values from the normal distribution
    let mut rng = thread_rng();
    let mut values = Vec::new();

    for _ in 0..1000 {
        let x = normal.sample(&mut rng);
        values.push(x);
    }

    // Calculate the mean and standard deviation of the sampled values
    let mean = values.iter().sum::<f64>() / (values.len() as f64);
    
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (values.len() as f64);
    
    let std_dev = variance.sqrt();

    // Print the mean and standard deviation
    println!("Mean: {}", mean);
    println!("Standard deviation: {}", std_dev);

