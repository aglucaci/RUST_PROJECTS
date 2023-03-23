use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // get the input filename from the command line arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // open the input file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // initialize counters for each nucleotide
    let mut a_count = 0;
    let mut c_count = 0;
    let mut g_count = 0;
    let mut t_count = 0;

    // iterate through each line of the input file
    for line in reader.lines() {
        let line = line.unwrap();
        // skip comment lines
        if line.starts_with('>') {
            continue;
        }
        // count the nucleotides in each sequence
        for c in line.chars() {
            match c {
                'A' | 'a' => a_count += 1,
                'C' | 'c' => c_count += 1,
                'G' | 'g' => g_count += 1,
                'T' | 't' => t_count += 1,
                _ => (),
            }
        }
    }

    // calculate the total number of nucleotides
    let total_count = a_count + c_count + g_count + t_count;

    // calculate the frequency of each nucleotide
    let a_freq = a_count as f64 / total_count as f64;
    let c_freq = c_count as f64 / total_count as f64;
    let g_freq = g_count as f64 / total_count as f64;
    let t_freq = t_count as f64 / total_count as f64;

    // print the results
    println!("A: {:.2}", a_freq);
    println!("C: {:.2}", c_freq);
    println!("G: {:.2}", g_freq);
    println!("T: {:.2}", t_freq);
}
