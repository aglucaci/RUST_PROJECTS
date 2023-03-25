fn inverse_matrix(matrix: [[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    if det == 0.0 {
        return None;
    }

    let inv_det = 1.0 / det;
    let mut result = [[0.0; 2]; 2];

    result[0][0] = matrix[1][1] * inv_det;
    result[0][1] = -matrix[0][1] * inv_det;
    result[1][0] = -matrix[1][0] * inv_det;
    result[1][1] = matrix[0][0] * inv_det;

    Some(result)
}

fn main() {
    let matrix = [[1.0, 2.0], [3.0, 4.0]];
    let result = inverse_matrix(matrix);

    match result {
        Some(inv_matrix) => println!("{:?}", inv_matrix),
        None => println!("Matrix is not invertible."),
    }
}

