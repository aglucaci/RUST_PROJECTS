
#[macro_use]
extern crate statrs;

use std::f64;
use statrs::statistics::Statistics;
use statrs::distribution::{Gamma, Continuous};

fn main() {
    let sequences = vec![
        "ATCGATCGATCG".to_string(),
        "ATCGATCGATCGATCG".to_string(),
        "ATCGATCGATCGATCGATCG".to_string(),
        "ATCGATCG".to_string(),
        "ATCGATCGATCGATCGATCGATCGATCGATCG".to_string(),
    ];
    
    let sequence_lengths: Vec<f64> = sequences.iter().map(|s| s.len() as f64).collect();
    let mean = sequence_lengths.clone().mean();
    let variance = sequence_lengths.variance();
    let alpha = mean * mean / variance;
    
    let gamma_dist = Gamma::new(alpha, 1.0).unwrap();
    
    println!("Shape parameter: {}", alpha);
    println!("Gamma distribution: {:?}", gamma_dist);
}

