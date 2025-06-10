use std::thread;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl Matrix {
    pub fn rows(&self) -> usize {
        self.0.len()
    }
    pub fn columns(&self) -> usize {
        self.0[0].len()
    }

    pub fn add_matrix(&self, other: &Matrix, operation_method: OperationMethod) -> Matrix {
        match operation_method {
            OperationMethod::SEQUENTIAL => self.add_seq(other),
            OperationMethod::PARALLEL => self.add_parall(other),
        }
    }
    fn add_parall(&self, other_matrix: &Matrix) -> Matrix {
        let rows = self.rows();
        let cols = self.columns();

        thread::scope(|s| {
            let threads: Vec<_> = (0..rows)
                .map(|i| {
                    s.spawn(move || {
                        (0..cols)
                            .map(|j| self.0[i][j] + other_matrix.0[i][j])
                            .collect()
                    })
                })
                .collect();
            Matrix(threads.into_iter().map(|t| t.join().unwrap()).collect())
        })
    }

    fn add_seq(&self, other_matrix: &Matrix) -> Matrix {
        let rows = self.rows();
        let cols = self.columns();
        let mut result = Vec::new();

        for i in 0..rows {
            let mut row = Vec::new();
            for j in 0..cols {
                row.push(self.0[i][j] + other_matrix.0[i][j]);
            }
            result.push(row);
        }
        Matrix(result)
    }
}

pub enum OperationMethod {
    SEQUENTIAL,
    PARALLEL,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn matrix_from_vec(v: Vec<Vec<f64>>) -> Matrix {
        Matrix(v)
    }

    // 1. 1×1 matrix
    #[test]
    fn test_add_1x1() {
        let a = matrix_from_vec(vec![vec![1.0]]);
        let b = matrix_from_vec(vec![vec![2.0]]);
        let expected = matrix_from_vec(vec![vec![3.0]]);

        assert_eq!(a.add_matrix(&b, OperationMethod::SEQUENTIAL), expected);
        assert_eq!(a.add_matrix(&b, OperationMethod::PARALLEL), expected);
    }

    // 2. 2×2 matrix
    #[test]
    fn test_add_2x2() {
        let a = matrix_from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = matrix_from_vec(vec![vec![4.0, 3.0], vec![2.0, 1.0]]);
        let expected = matrix_from_vec(vec![vec![5.0, 5.0], vec![5.0, 5.0]]);

        assert_eq!(a.add_matrix(&b, OperationMethod::SEQUENTIAL), expected);
        assert_eq!(a.add_matrix(&b, OperationMethod::PARALLEL), expected);
    }

    // 3. Non‐square matrix (3×5)
    #[test]
    fn test_add_non_square() {
        let a = matrix_from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0, 10.0],
            vec![11.0, 12.0, 13.0, 14.0, 15.0],
        ]);
        let b = matrix_from_vec(vec![
            vec![15.0, 14.0, 13.0, 12.0, 11.0],
            vec![10.0, 9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0, 1.0],
        ]);
        let expected = matrix_from_vec(vec![
            vec![16.0, 16.0, 16.0, 16.0, 16.0],
            vec![16.0, 16.0, 16.0, 16.0, 16.0],
            vec![16.0, 16.0, 16.0, 16.0, 16.0],
        ]);

        assert_eq!(a.add_matrix(&b, OperationMethod::SEQUENTIAL), expected);
        assert_eq!(a.add_matrix(&b, OperationMethod::PARALLEL), expected);
    }

    // 4. Negative values
    #[test]
    fn test_add_negative_values() {
        let a = matrix_from_vec(vec![vec![-1.0, -2.0], vec![-3.5, 4.0]]);
        let b = matrix_from_vec(vec![vec![1.0, 2.5], vec![3.5, -4.0]]);
        let expected = matrix_from_vec(vec![vec![0.0, 0.5], vec![0.0, 0.0]]);

        assert_eq!(a.add_matrix(&b, OperationMethod::SEQUENTIAL), expected);
        assert_eq!(a.add_matrix(&b, OperationMethod::PARALLEL), expected);
    }

    // 5. Zero matrix
    #[test]
    fn test_add_zero_matrix() {
        let rows = 4;
        let cols = 3;
        let zero = vec![vec![0.0; cols]; rows];
        let a = matrix_from_vec(zero.clone());
        let b = matrix_from_vec(zero.clone());
        let expected = matrix_from_vec(zero);

        assert_eq!(a.add_matrix(&b, OperationMethod::SEQUENTIAL), expected);
        assert_eq!(a.add_matrix(&b, OperationMethod::PARALLEL), expected);
    }

    // 6. Fractional values
    #[test]
    fn test_add_fractional_values() {
        let a = matrix_from_vec(vec![vec![1.1, 2.2, 3.3], vec![4.4, 5.5, 6.6]]);
        let b = matrix_from_vec(vec![vec![0.9, 0.8, 0.7], vec![0.6, 0.5, 0.4]]);
        let expected = matrix_from_vec(vec![vec![2.0, 3.0, 4.0], vec![5.0, 6.0, 7.0]]);

        assert_eq!(a.add_matrix(&b, OperationMethod::SEQUENTIAL), expected);
        assert_eq!(a.add_matrix(&b, OperationMethod::PARALLEL), expected);
    }

    // 7. Original matrices remain unchanged
    #[test]
    fn test_original_unchanged() {
        let a = matrix_from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = matrix_from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let a_clone = a.clone();
        let b_clone = b.clone();

        let _ = a.add_matrix(&b, OperationMethod::SEQUENTIAL);
        let _ = a.add_matrix(&b, OperationMethod::PARALLEL);

        assert_eq!(a, a_clone);
        assert_eq!(b, b_clone);
    }

    // 8. Equivalence on random 10×10 matrix
    #[test]
    fn test_equivalence_random_10x10() {
        let mut v1 = Vec::with_capacity(10);
        let mut v2 = Vec::with_capacity(10);
        for i in 0..10 {
            let row1: Vec<f64> = (0..10).map(|j| (i * 10 + j) as f64).collect();
            let row2: Vec<f64> = (0..10).map(|j| ((i * 10 + j) as f64) * 2.0).collect();
            v1.push(row1);
            v2.push(row2);
        }
        let a = matrix_from_vec(v1.clone());
        let b = matrix_from_vec(v2.clone());
        let seq = a.add_matrix(&b, OperationMethod::SEQUENTIAL);
        let par = a.add_matrix(&b, OperationMethod::PARALLEL);

        // check all entries
        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(seq.0[i][j], par.0[i][j]);
            }
        }
    }

    // 9. Mismatched dimensions should panic (if desired)
    #[test]
    #[should_panic]
    fn test_mismatched_dimensions() {
        let a = matrix_from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let b = matrix_from_vec(vec![vec![1.0], vec![2.0]]);
        // Assuming your implementation will panic on dimension mismatch
        let _ = a.add_matrix(&b, OperationMethod::SEQUENTIAL);
    }

    // 10. Performance: parallel faster than sequential for large matrix
    #[test]
    fn test_parallel_faster_than_sequential() {
        let rows = 500;
        let cols = 500;
        let a_data = vec![vec![1.0; cols]; rows];
        let b_data = vec![vec![2.0; cols]; rows];
        let a = matrix_from_vec(a_data.clone());
        let b = matrix_from_vec(b_data.clone());

        let start_seq = Instant::now();
        let _ = a.add_matrix(&b, OperationMethod::SEQUENTIAL);
        let dur_seq = start_seq.elapsed();

        let start_par = Instant::now();
        let _ = a.add_matrix(&b, OperationMethod::PARALLEL);
        let dur_par = start_par.elapsed();

        assert!(
            dur_par < dur_seq,
            "Parallel ({:?}) should be faster than sequential ({:?})",
            dur_par,
            dur_seq
        );
    }
}
