pub mod problem1;
pub mod problem2;

#[cfg(test)]

mod tests {
    use crate::problem1::{sum, dedup, filter};

    use crate::problem2::mat_mult;
    // use crate::problem3::sieve;
    // use crate::problem4::{hanoi, Peg};

    #[test]
    fn test_sum_small() {
        let array = [1,2,3,4,5];
        assert_eq!(sum(&array), 15);
    }

    #[test]
    fn test_dedup_small() {
        let vs = vec![1,2,2,3,4,1];
        assert_eq!(dedup(&vs), vec![1,2,3,4]);
    }

    fn even_predicate(x: i32) -> bool {
        (x % 2) == 0
    }

    #[test]
    fn test_filter_small() {
        let vs = vec![1,2,3,4,5];
        assert_eq!(filter(&vs, &even_predicate), vec![2,4]);
    }

    #[test]
    fn test_mat_mult_identity() {
        let mut mat1 = vec![vec![0.;3]; 3];
        for i in 0..mat1.len() {
            mat1[i][i] = 1.;
        }
        let mat2 = vec![vec![5.;3]; 3];
        let result = mat_mult(&mat1, &mat2);
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                assert_eq!(result[i][j], mat2[i][j]);
            }
        }
    }
}