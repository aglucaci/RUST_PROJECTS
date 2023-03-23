use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get the input file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];

    // Open the input file
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);

    // Create a hash map to store the nucleotide frequencies at each position
    let mut freqs: HashMap<usize, HashMap<char, f64>> = HashMap::new();

    // Loop over the lines in the input file
    let mut seq_idx = 0;
    let mut line_idx = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        // If this is a header line, skip it
        if line.starts_with('>') {
            seq_idx += 1;
            line_idx = 0;
            continue;
        }

        // Loop over the codons in the sequence
        let mut codon_idx = 0;
        while codon_idx < line.len() / 3 {
            // Get the next codon
            let codon = &line[codon_idx*3..(codon_idx+1)*3];

            // Increment the count for each nucleotide in the codon
            for (nuc_idx, nuc) in codon.chars().enumerate() {
                let pos_idx = line_idx*3 + nuc_idx;
                let pos_freqs = freqs.entry(pos_idx).or_insert(HashMap::new());
                *pos_freqs.entry(nuc).or_insert(0.0) += 1.0 / (seq_idx as f64);
            }

            codon_idx += 1;
        }

        line_idx += 1;
    }

    // Scale the frequencies at each position to sum to 1.0
    let pos_idxs: Vec<usize> = freqs.keys().cloned().collect();
    let max_pos_idx = pos_idxs.iter().max().unwrap();
    for pos_idx in 0..=*max_pos_idx {
        if let Some(pos_freqs) = freqs.get(&pos_idx) {
            let total_freq: f64 = pos_freqs.values().sum();
            let scaled_freqs: HashMap<char, f64> = pos_freqs.iter()
                .map(|(nuc, count)| (*nuc, count / total_freq))
                .collect();

            // Print the scaled nucleotide frequencies at the position
            print!("Position {}: ", pos_idx);
            for nuc in "ACGT".chars() {
                let count = scaled_freqs.get(&nuc).unwrap_or(&0.0);
                print!("{}: {:.4}, ", nuc, count);
            }
            println!();
        }
    }
}
