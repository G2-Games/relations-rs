use std::fmt::Display;

fn main() {
    let input_matrix = [
        [0, 1, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [0, 1, 1, 0],
    ];

    let matrix = Matrix::new(input_matrix);

    println!("Input Matrix:");
    println!("{}", matrix);
    println!();

    dbg!(matrix.is_reflexive());
    dbg!(matrix.is_irreflexive());
    dbg!(matrix.is_symmetric());
    dbg!(matrix.is_antisymmetric());
    dbg!(matrix.is_asymmetric());
    dbg!(matrix.is_transitive());
}

struct Matrix<const S: usize> {
    matrix: [[u8; S]; S],
}

impl<const S: usize> Display for Matrix<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = Vec::new();
        for row in &self.matrix {
            rows.push(format!("{:?}", row))
        }
        let rows = rows.join("\n");
        write!(f, "{}", rows)
    }
}

impl<const S: usize> Matrix<S> {
    /// Create a new square matrix
    fn new(matrix: [[u8; S]; S]) -> Self {
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
        for i in 0..self.len() {
            if self.matrix[i][i] == 0 {
                return false
            }
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
        if self.is_antisymmetric() && self.is_irreflexive() {
            true
        } else {
            false
        }
    }

    /// Tests if the given matrix is transitive
    fn is_transitive(&self) -> bool {
        let length = self.len();
        let mut output = self.matrix.clone();
        for k in 0..length {
            for i in 0..length {
                for j in 0..length {
                    output[i][j] = output[i][j] | (output[i][k] & output[k][j]);
                    if output[i][j] != self.matrix[i][j] {
                        return false
                    }
                }
            }
        }

        /*
        for row in &output {
            println!("{:?}", row);
        }
        */

        true
    }
}

#[cfg(test)]
mod matrix {
    use crate::Matrix;

    #[test]
    fn reflexive() {
        assert!(
            Matrix::new([
                [1, 1, 1, 0],
                [0, 1, 0, 1],
                [0, 0, 1, 1],
                [0, 0, 0, 1]
            ]).is_reflexive()
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
            ]).is_irreflexive()
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
            ]).is_symmetric()
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
            ]).is_antisymmetric()
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
            ]).is_asymmetric()
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
            ]).is_transitive()
        );
    }
}
