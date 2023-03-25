fn subtract_matrix(matrix1: [[i32; 2]; 2], matrix2: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    let mut result = [[0; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            result[i][j] = matrix1[i][j] - matrix2[i][j];
        }
    }

    result
}

fn main() {
    let matrix1 = [[1, 2], [3, 4]];
    let matrix2 = [[5, 6], [7, 8]];
    let result = subtract_matrix(matrix1, matrix2);

    println!("{:?}", result);
}

