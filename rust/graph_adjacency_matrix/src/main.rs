use std::io;
use std::io::prelude::*;

fn get_number<T>(stdin: &io::Stdin) -> Option<T>
    where T: std::str::FromStr
{
    let mut s = String::new();
    match stdin.read_line(&mut s) {
        Err(_) => None,
        Ok(_) => s.trim().parse::<T>().ok(),
    }
}

fn print_square_matrix<T: std::fmt::Display>(matrix: &[T], size: usize) {
    for i in 0..size {
        for j in 0..size {
            print!("{} ", matrix[i * size + j]);
        }
        println!("");
    }
}

fn main() {
    let stdin = io::stdin();
    let n: usize = get_number(&stdin).unwrap();

    let mut adjacency_matrix = vec![0; n * n];
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let neighbors = line.split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect::<Vec<usize>>();

        let i = neighbors[0] - 1;
        let j = neighbors[1] - 1;
        adjacency_matrix[i * n + j] += 1;
        adjacency_matrix[j * n + i] += 1;
    }
    print_square_matrix(&adjacency_matrix, n);
}
