fn main() {
    let input_matrix = vec![
        vec![1, 1, 0, 0],
        vec![1, 1, 0, 1],
        vec![0, 0, 1, 0],
        vec![0, 1, 0, 1]
    ];

    println!("Input Matrix:");
    for row in &input_matrix {
        println!("{:?}", row);
    }
    println!();

    dbg!(is_reflexive(&input_matrix));
    dbg!(is_irreflexive(&input_matrix));
    dbg!(is_symmetric(&input_matrix));
}

/// Tests if the given matrix is reflexive
fn is_reflexive(matrix: &Vec<Vec<u8>>) -> bool {
    for i in 0..matrix.len() {
        if matrix[i][i] == 0 {
            return false
        }
    }

    true
}

/// Tests if the given matrix is irreflexive
fn is_irreflexive(matrix: &Vec<Vec<u8>>) -> bool {
    for i in 0..matrix.len() {
        if matrix[i][i] == 1 {
            return false
        }
    }

    true
}

/// Tests if the given matrix is symmetric
fn is_symmetric(matrix: &Vec<Vec<u8>>) -> bool {
    for i in 0..matrix.len() {
        for x in 0..matrix.len() {
            if matrix[x][i] != matrix[i][x] {
                return false
            }
        }
    }

    true
}

fn is_antisymmetric(matrix: &Vec<Vec<u8>>) -> bool {

}
