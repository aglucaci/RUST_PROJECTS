fn proportional_distance(seq1: &str, seq2: &str) -> Option<f64> {
    if seq1.len() != seq2.len() {
        return None;
    }

    let mut num_diff = 0.0;
    let mut num_same = 0.0;

    for (a, b) in seq1.chars().zip(seq2.chars()) {
        if a == b {
            num_same += 1.0;
        } else {
            num_diff += 1.0;
        }
    }

    Some(num_diff / (num_same + num_diff))
}

fn pairwise_proportional_distances(sequences: &[&str]) -> Vec<Vec<Option<f64>>> {
    let n = sequences.len();
    let mut matrix = vec![vec![None; n]; n];

    for i in 0..n {
        for j in (i + 1)..n {
            let distance = proportional_distance(sequences[i], sequences[j]);

            matrix[i][j] = distance;
            matrix[j][i] = distance;
        }
    }

    matrix
}

fn main() {
    let sequences = vec!["ACTGACTGAC", "CCTGACTGAC", "ACTGACCAGT"];
    let matrix = pairwise_proportional_distances(&sequences);

    println!("Pairwise proportional distance matrix:");

    for row in matrix.iter() {
        for item in row.iter() {
            match item {
                Some(distance) => print!("{:.3} ", distance),
                None => print!("X "),
            }
        }

        println!();
    }
}

