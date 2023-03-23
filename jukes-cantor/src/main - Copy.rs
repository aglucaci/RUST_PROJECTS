use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Define a function to calculate the Jukes-Cantor distance between two sequences
fn jukes_cantor(seq1: &str, seq2: &str) -> f64 {
    let mut diff_count = 0;
    let mut total_count = 0;

    // Iterate over each codon in the sequences
    for (codon1, codon2) in seq1.chars().zip(seq2.chars()).step_by(3) {
        // Ignore any codons that contain an ambiguous nucleotide
        if codon1 == 'N' || codon2 == 'N' {
            continue;
        }

        total_count += 1;

        // If the codons differ, increment the difference count
        if codon1 != codon2 {
            diff_count += 1;
        }
    }

    // If there are no differences, the distance is 0
    if diff_count == 0 {
        return 0.0;
    }

    // Calculate the Jukes-Cantor distance using the maximum likelihood method
    let p = diff_count as f64 / total_count as f64;
    -3.0 / 4.0 * ((1.0 - 4.0 / 3.0 * p).ln())
}

fn main() {
    // Read the input file path from the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please provide a file path for the input FASTA file.");
    }

    // Open the input file and read in the sequences
    let file = File::open(&args[1]).expect("Unable to open input file.");
    let mut sequences: Vec<String> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        if !line.starts_with('>') {
            sequences.push(line);
        }
    }

    // Calculate the Jukes-Cantor distances between each pair of sequences
    let num_sequences = sequences.len();

    for i in 0..num_sequences {
        for j in (i + 1)..num_sequences {
            let dist = jukes_cantor(&sequences[i], &sequences[j]);
            println!("{} -> {}: {}", i + 1, j + 1, dist);
        }
    }
}
