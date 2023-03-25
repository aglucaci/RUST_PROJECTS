fn transpose_matrix(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
        }
    }

    result
}

fn main() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let result = transpose_matrix(matrix);

    println!("{:?}", result);
}

