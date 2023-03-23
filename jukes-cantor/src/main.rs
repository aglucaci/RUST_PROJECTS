use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please provide a file path for the input FASTA file.");
    }

    let file = File::open(&args[1]).expect("Unable to open input file.");

    let mut sequences: Vec<String> = Vec::new();
    let mut current_sequence: String = String::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        if line.starts_with('>') {
            if !current_sequence.is_empty() {
                sequences.push(current_sequence);
                current_sequence = String::new();
            }
        } else {
            current_sequence.push_str(&line);
        }
    }

    sequences.push(current_sequence);

    let num_sequences = sequences.len();
    let seq_length = sequences[0].len() / 3;

    let mut distances: Vec<Vec<f64>> = vec![vec![0.0; num_sequences]; num_sequences];

    for i in 0..num_sequences {
        for j in (i + 1)..num_sequences {
            let seq1 = &sequences[i];
            let seq2 = &sequences[j];

            let mut diffs = 0;

            for k in 0..seq_length {
                let codon1 = &seq1[(k * 3)..((k + 1) * 3)];
                let codon2 = &seq2[(k * 3)..((k + 1) * 3)];

                if codon1 != codon2 {
                    diffs += 1;
                }
            }

            let p = diffs as f64 / seq_length as f64;

            let d = -3.0 / 4.0 * (1.0 - (4.0 / 3.0) * p).ln();

            distances[i][j] = d;
            distances[j][i] = d;
        }
    }

    for row in distances {
        let output: Vec<String> = row.iter().map(|&x| x.to_string()).collect();
        println!("{}", output.join("\t"));
    }
}
