fn hamming_distance(seq1: &str, seq2: &str) -> Option<usize> {
    if seq1.len() != seq2.len() {
        return None;
    }

    Some(seq1.chars().zip(seq2.chars()).filter(|&(a, b)| a != b).count())
}

fn pairwise_hamming_distances(sequences: &[&str]) -> Vec<Vec<Option<usize>>> {
    let n = sequences.len();
    let mut matrix = vec![vec![None; n]; n];

    for i in 0..n {
        for j in (i + 1)..n {
            let distance = hamming_distance(sequences[i], sequences[j]);

            matrix[i][j] = distance;
            matrix[j][i] = distance;
        }
    }

    matrix
}

fn main() {
    let sequences = vec!["ACTGACTGAC", "CCTGACTGAC", "ACTGACCAGT"];
    let matrix = pairwise_hamming_distances(&sequences);

    println!("Pairwise Hamming distance matrix:");

    for row in matrix.iter() {
        for item in row.iter() {
            match item {
                Some(distance) => print!("{} ", distance),
                None => print!("X "),
            }
        }

        println!();
    }
}

