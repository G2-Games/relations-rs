# Relations - Discrete Structures
This program calculates if a matrix conforms to certain relations, and also 
can calculate how to transform a matrix to conform to the relation. It is 
capable of processing all properties of a matrix of 10 billion pairs in around
6 seconds.

## Clone
To get started, clone the repository using Git and enter the directory, and
then follow the steps under [running](https://github.com/G2-Games/relations-rs?tab=readme-ov-file#running)
```
git clone https://github.com/G2-Games/relations-rs.git
cd relations-rs
```

## Running
Simply use the following command to compile and run the program:
```
cargo run
```

The input matrix can be adjusted by modifying the matrix at the beginning of
the `main` function. Ensure it is square, otherwise it will not compile.
```rust
// Modify this matrix
let input_matrix = [
    [1, 0, 0, 1],
    [0, 1, 1, 0],
    [0, 1, 1, 0],
    [1, 0, 1, 1]
];
```

## Testing
To run the unit tests, use the command:
```
cargo test
```
## Example Outputs
![image](https://github.com/G2-Games/relations-rs/assets/72430668/2306f2de-c3fe-4554-95f1-e5ca55a49e07)
![image](https://github.com/G2-Games/relations-rs/assets/72430668/7d424ba7-abff-47df-a8e8-3c6861fb787d)
![image](https://github.com/G2-Games/relations-rs/assets/72430668/76448ba6-3f6c-4e5d-b799-e252b41efd3b)

