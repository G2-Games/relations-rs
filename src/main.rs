use std::{fmt::Display, thread::sleep, time::{Duration, Instant}};
use rayon::prelude::*;

fn main() {
    let input_matrix = [
       //a, b, c
        [0, 1, 1],//a
        [0, 0, 1],//b
        [0, 0, 0],//c
    ];

    let matrix = Matrix::new(input_matrix.into());

    println!("----------");
    let timer = Instant::now();
    //let rand: Matrix<10_000> = Matrix::new_random();
    let rand = &matrix;

    println!("{}x{} Matrix:", rand.len(), rand.len());
    if rand.len() < 10 {
        println!("{}", rand);
    } else {
        println!("NOT PRINTING; size >= 10x10");
    }

    println!("Constructing random matrix took {:#?}", timer.elapsed());

    let timer = Instant::now();
    println!("Reflexive:     {} in {:#?}", rand.is_reflexive(), timer.elapsed());

    let timer = Instant::now();
    println!("Irreflexive:   {} in {:#?}", rand.is_irreflexive(), timer.elapsed());

    let timer = Instant::now();
    println!("Symmetric:     {} in {:#?}", rand.is_symmetric(), timer.elapsed());

    let timer = Instant::now();
    println!("Antisymmetric: {} in {:#?}", rand.is_antisymmetric(), timer.elapsed());

    let timer = Instant::now();
    println!("Asymmetric:    {} in {:#?}", rand.is_asymmetric(), timer.elapsed());

    let timer = Instant::now();
    println!("Transitive:    {} in {:#?}", rand.is_transitive(), timer.elapsed());
}

struct Matrix<const S: usize> {
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
    fn new(matrix: Box<[[u8; S]; S]>) -> Self {
        Self {
            matrix
        }
    }

    /// Create a new square matrix initalized with random values
    fn new_random() -> Self {
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
        // Use Warshall's algorithm to determine transitivity
        for i in 0..self.len() {
            for j in 0..self.len() {
                for k in 0..self.len() {
                    // Determine if the new value in the output matrix is the
                    // same as the input, if not the original is NOT transitive
                    if self.matrix[j][k] != self.matrix[j][k] | (self.matrix[j][i] & self.matrix[i][k]) {
                        return false
                    }
                }
            }
        }

        // If never returned false, then it must be true
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
}
