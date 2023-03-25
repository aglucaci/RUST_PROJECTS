//use statrs::distribution::{Gamma, Univariate};
use statrs::statistics::{Max, Mean};
use std::error::Error;
use statrs::distribution::{Gamma, Continuous};
//use statrs::statistics::Mean;
//use statrs::prec;

fn gamma_shape_parameter(sequences: &[String]) -> Result<f64, Box<dyn Error>> {
    let sequence_lengths: Vec<usize> = sequences.iter().map(|s| s.len()).collect();
    let max = sequence_lengths.iter().max().unwrap_or(&0);
    let mean = sequence_lengths.iter().mean();
    let s2 = sequence_lengths.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / (sequence_lengths.len() - 1) as f64;
    let alpha = (mean / s2).powi(2) / (1.0 / s2);

    Ok(alpha)
}

fn main() -> Result<(), Box<dyn Error>> {
    let sequences = vec![
        "ATCGATCGATCG".to_string(),
        "ATCGATCGATCGATCG".to_string(),
        "ATCGATCGATCGATCGATCG".to_string(),
        "ATCGATCG".to_string(),
        "ATCGATCGATCGATCGATCGATCGATCGATCG".to_string(),
    ];

    let alpha = gamma_shape_parameter(&sequences)?;

    println!("Shape parameter (alpha): {}", alpha);

    Ok(())
}

