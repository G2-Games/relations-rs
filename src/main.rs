use std::fmt::Display;
use rayon::prelude::*;

fn main() {
    let input_matrix = [
        [1, 0, 0, 1],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [1, 0, 0, 1]
    ];
    let matrix = Matrix::new(input_matrix.into());

    println!("Input matrix:\n{}\n-------", matrix);

    println!("This matrix is...");

    print!("    Reflexive: ");
    match matrix.is_reflexive() {
        true => println!("[\x1b[0;32m✓\x1b[0m]"),
        false => {
            println!("[\x1b[0;31mX\x1b[0m]");
            println!("Computed reflexive closure:");
            println!("{}", matrix.make_reflexive());
        }
    }

    print!("  Irreflexive: ");
    match matrix.is_irreflexive() {
        true => println!("[\x1b[0;32m✓\x1b[0m]"),
        false => println!("[\x1b[0;31mX\x1b[0m]"),
    }

    print!("    Symmetric: ");
    match matrix.is_symmetric() {
        true => println!("[\x1b[0;32m✓\x1b[0m]"),
        false => {
            println!("[\x1b[0;31mX\x1b[0m]");
            println!("Computed symmetric closure:");
            println!("{}", matrix.make_symmetric());
        }
    }

    print!("Antisymmetric: ");
    match matrix.is_antisymmetric() {
        true => println!("[\x1b[0;32m✓\x1b[0m]"),
        false => println!("[\x1b[0;31mX\x1b[0m]"),
    }

    print!("   Asymmetric: ");
    match matrix.is_asymmetric() {
        true => println!("[\x1b[0;32m✓\x1b[0m]"),
        false => println!("[\x1b[0;31mX\x1b[0m]"),
    }

    print!("   Transitive: ");
    match matrix.is_transitive() {
        true => println!("[\x1b[0;32m✓\x1b[0m]"),
        false => {
            println!("[\x1b[0;31mX\x1b[0m]");
            println!("Computed transitive closure:");
            println!("{}", matrix.make_transitive());
        }
    }

    print!("  Equivalence: ");
    match matrix.is_equivalence() {
        true => println!("[\x1b[0;32m✓\x1b[0m]"),
        false => println!("[\x1b[0;31mX\x1b[0m]"),
    }
}

pub struct Matrix<const S: usize> {
    matrix: Box<[[u8; S]; S]>,
}

impl<const S: usize> Display for Matrix<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = Vec::new();
        for row in *self.matrix {
            rows.push(format!("{:?}", row))
        }
        let rows = rows.join("\n");
        write!(f, "{}", rows)
    }
}

impl<const S: usize> Matrix<S> {
    /// Create a new square matrix
    const fn new(matrix: Box<[[u8; S]; S]>) -> Self {
        Self {
            matrix
        }
    }

    /// Create a new square matrix initalized with random values
    pub fn new_random() -> Self {
        let mut matrix = Box::new([
            [0; S]; S
        ]);

        matrix.par_iter_mut().for_each(|row| {
            for i in row.iter_mut() {
                *i = fastrand::bool().into()
            }
        });

        Self {
            matrix
        }
    }

    #[inline]
    const fn len(&self) -> usize {
        self.matrix.len()
    }

    /// Tests if the given matrix is reflexive
    fn is_reflexive(&self) -> bool {
        let mut i = 0;
        while i < self.len() {
            if self.matrix[i][i] == 0 {
                return false
            }
            i += 1
        }

        true
    }

    /// Tests if the given matrix is irreflexive
    fn is_irreflexive(&self) -> bool {
        for i in 0..self.len() {
            if self.matrix[i][i] == 1 {
                return false
            }
        }

        true
    }

    /// Tests if the given matrix is symmetric
    fn is_symmetric(&self) -> bool {
        for i in 0..self.len() {
            for x in 0..self.len() {
                if self.matrix[x][i] != self.matrix[i][x] {
                    return false
                }
            }
        }

        true
    }

    /// Tests if the given matrix is antisymmetric
    fn is_antisymmetric(&self) -> bool {
        for i in 0..self.len() {
            for x in 0..self.len() {
                if x == i {
                    continue
                }
                if self.matrix[x][i] & self.matrix[i][x] != 0 {
                    return false
                }
            }
        }

        true
    }

    /// Tests if the given matrix is asymmetric
    fn is_asymmetric(&self) -> bool {
        for i in 0..self.len() {
            for x in 0..self.len() {
                if self.matrix[x][i] & self.matrix[i][x] != 0 {
                    return false
                }
            }
        }

        true
    }

    /// Tests if the given matrix is transitive
    fn is_transitive(&self) -> bool {
        // Use a parallel implementation of Warshall's algorithm to determine
        // transitivity
        (0..self.len()).into_par_iter().try_for_each(|i| {
            for j in 0..self.len() {
                for k in 0..self.len() {
                    if self.matrix[j][k] != self.matrix[j][k] | (self.matrix[j][i] & self.matrix[i][k]) {
                        return None
                    }
                }
            }
            Some(())
        }).is_some()
    }

    /// Tests if the matrix is an equivalence relation
    fn is_equivalence(&self) -> bool {
        self.is_reflexive()
            && self.is_symmetric()
            && self.is_transitive()
    }

    /// Makes the current matrix reflexive by computing its reflexive closure
    fn make_reflexive(&self) -> Self {
        let mut output_matrix = self.matrix.clone();

        for i in 0..output_matrix.len() {
            output_matrix[i][i] = 1
        }

        Self {
            matrix: output_matrix
        }
    }

    /// Makes the current matrix symmetric by computing its symmetric closure
    fn make_symmetric(&self) -> Self {
        let mut output_matrix = self.matrix.clone();

        for i in 0..output_matrix.len() {
            for x in 0..output_matrix.len() {
                if output_matrix[x][i] != output_matrix[i][x] {
                    (output_matrix[x][i], output_matrix[i][x]) = (1, 1)
                }
            }
        }

        Self {
            matrix: output_matrix
        }
    }

    /// Makes the current matrix transitive by computing its transitive closure
    fn make_transitive(&self) -> Self {
        let mut output_matrix = self.matrix.clone();

        for k in 0..output_matrix.len() {
            for i in 0..output_matrix.len() {
                let v2 = output_matrix[i][k];
                for j in 0..output_matrix.len() {
                    output_matrix[i][j] |= v2 & output_matrix[k][j];
                }
            }
        }

        Self {
            matrix: output_matrix
        }
    }
}

#[cfg(test)]
mod matrix {
    use crate::Matrix;

    /* Checking that something is */

    #[test]
    fn reflexive() {
        assert!(
            Matrix::new([
                [1, 1, 1, 0],
                [0, 1, 0, 1],
                [0, 0, 1, 1],
                [0, 0, 0, 1]
            ].into()).is_reflexive()
        );
    }

    #[test]
    fn irreflexive() {
        assert!(
            Matrix::new([
                [0, 1, 1, 1],
                [1, 0, 1, 1],
                [1, 1, 0, 1],
                [0, 1, 1, 0]
            ].into()).is_irreflexive()
        );
    }

    #[test]
    fn symmetric() {
        assert!(
            Matrix::new([
                [0, 1, 1, 1],
                [1, 0, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 0]
            ].into()).is_symmetric()
        );
    }

    #[test]
    fn antisymmetric() {
        assert!(
            Matrix::new([
                [0, 0, 0, 1],
                [1, 0, 0, 0],
                [1, 1, 0, 1],
                [0, 1, 0, 1]
            ].into()).is_antisymmetric()
        );
    }

    #[test]
    fn asymmetric() {
        assert!(
            Matrix::new([
                [0, 0, 0, 1],
                [1, 0, 0, 0],
                [1, 1, 0, 1],
                [0, 1, 0, 0]
            ].into()).is_asymmetric()
        );
    }

    #[test]
    fn transitive() {
        assert!(
            Matrix::new([
                [1, 1, 1, 1],
                [0, 1, 0, 0],
                [0, 1, 1, 1],
                [0, 0, 0, 0]
            ].into()).is_transitive()
        );
    }

    #[test]
    fn equivalence() {
        let matrix = Matrix::new([
            [1, 0, 0, 0],
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 1]
        ].into());

        // If it's all of these...
        assert!(matrix.is_reflexive());
        assert!(matrix.is_symmetric());
        assert!(matrix.is_transitive());

        // Then it's also an equivalence relation!
        assert!(matrix.is_equivalence());
    }

    /* Checking that something is not */

    #[test]
    fn not_reflexive() {
        assert!(
            !Matrix::new([
                [1, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 1],
                [0, 0, 0, 1]
            ].into()).is_reflexive()
        );
    }

    #[test]
    fn not_irreflexive() {
        assert!(
            !Matrix::new([
                [0, 1, 1, 1],
                [1, 0, 1, 1],
                [1, 1, 1, 1],
                [0, 1, 1, 0]
            ].into()).is_irreflexive()
        );
    }

    #[test]
    fn not_symmetric() {
        assert!(
            !Matrix::new([
                [0, 1, 1, 1],
                [1, 0, 1, 1],
                [0, 1, 1, 1],
                [1, 1, 1, 0]
            ].into()).is_symmetric()
        );
    }

    #[test]
    fn not_antisymmetric() {
        assert!(
            !Matrix::new([
                [0, 0, 0, 1],
                [1, 0, 1, 0],
                [1, 1, 0, 1],
                [0, 1, 0, 1]
            ].into()).is_antisymmetric()
        );
    }

    #[test]
    fn not_asymmetric() {
        assert!(
            !Matrix::new([
                [0, 0, 0, 1],
                [1, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 1, 0, 0]
            ].into()).is_asymmetric()
        );
    }

    #[test]
    fn not_transitive() {
        assert!(
            !Matrix::new([
                [1, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 1],
                [0, 0, 0, 1]
            ].into()).is_transitive()
        );
    }

    #[test]
    fn not_equivalence() {
        assert!(
            !Matrix::new([
                [1, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 1],
                [0, 0, 0, 1]
            ].into()).is_reflexive()
        );
    }

    /* Computing Closure Tests */

    #[test]
    fn make_reflexive() {
        let matrix = Matrix::new([
            [1, 1, 1, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 1],
            [0, 0, 0, 1]
        ].into());

        // It isn't reflexive
        assert!(!matrix.is_reflexive());

        let closure = matrix.make_reflexive();

        // Now it is!
        assert!(closure.is_reflexive());
    }

    #[test]
    fn make_symmetric() {
        let matrix = Matrix::new([
            [0, 1, 1, 1],
            [1, 0, 1, 1],
            [0, 1, 1, 1],
            [1, 1, 1, 0]
        ].into());

        // It isn't symmetric
        assert!(!matrix.is_symmetric());

        let closure = matrix.make_symmetric();

        // Now it is!
        assert!(closure.is_symmetric());
    }

    #[test]
    fn make_transitive() {
        let matrix = Matrix::new([
            [1, 1, 1, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 1],
            [0, 0, 0, 1]
        ].into());

        // It isn't symmetric
        assert!(!matrix.is_transitive());

        let closure = matrix.make_transitive();

        // Now it is!
        assert!(closure.is_transitive());
    }

    /* Edge cases */

    #[test]
    fn all_zeroes() {
        let matrix = Matrix::new([
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1]
        ].into());

        assert!(matrix.is_reflexive());
        assert!(matrix.is_symmetric());
        assert!(matrix.is_transitive());
        assert!(matrix.is_equivalence());
    }

    #[test]
    fn all_ones() {
        let matrix = Matrix::new([
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1],
            [1, 1, 1, 1]
        ].into());

        assert!(matrix.is_reflexive());
        assert!(matrix.is_symmetric());
        assert!(matrix.is_transitive());
        assert!(matrix.is_equivalence());
    }

    #[test]
    fn one_by_one() {
        let matrix = Matrix::new([
            [0]
        ].into());

        assert!(!matrix.is_reflexive());
        assert!(matrix.is_symmetric());
        assert!(matrix.is_transitive());
        assert!(!matrix.is_equivalence());
    }
}
