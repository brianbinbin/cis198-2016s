/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let rows = mat1.len();
    let cols = mat2[0].len();
    let mut result = vec![vec![0.;rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            let mut sum = 0.;
            for k in 0..mat1[i].len() {
                sum += mat1[i][k] * mat2[k][i];
            }
            result[i][j] = sum;
        }
    }

    result
}


