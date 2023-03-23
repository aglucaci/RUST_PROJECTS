
use bio::io::fasta;
//use bio::alignment::pairwise::dna;
use bio::alphabets::dna;
use bio::alignment::pairwise;
use bio::alignment::Alignment;
use bio::phylo::{DistanceMatrix, nj_tree};

fn main() {
    // Get the input file path from the command line arguments
    let args: Vec<String> = std::env::args().collect();
    let input_path = &args[1];

    // Parse the input FASTA file
    let mut sequences = Vec::new();
    let reader = fasta::Reader::from_file(input_path).unwrap();
    for result in reader.records() {
        let record = result.unwrap();
        let seq = String::from_utf8(record.seq().to_vec()).unwrap();
        sequences.push(seq);
    }

    // Calculate the pairwise distances between sequences
    let mut distances = DistanceMatrix::new(&sequences);
    for i in 0..sequences.len() {
        for j in i+1..sequences.len() {
            let align = Alignment::with_capacity(sequences[i].len(), sequences[j].len());
            let score = dna::align(&sequences[i], &sequences[j], &align);
            let dist = (1.0 - (score / align.score() as f64)).max(0.0); // Jukes-Cantor distance
            distances.set_distance(i, j, dist);
        }
    }

    // Build the neighbor-joining tree
    let tree = nj_tree(&distances);

    // Print the Newick string of the tree
    println!("{}", tree.newick());
}
