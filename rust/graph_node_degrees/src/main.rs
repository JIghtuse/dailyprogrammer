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

fn main() {
    let stdin = io::stdin();
    let n: usize = get_number(&stdin).unwrap();

    let mut node_degrees = vec![0; n];
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let neighbors = line.split_whitespace()
                          .map(|s| s.parse().unwrap())
                          .collect::<Vec<usize>>();

        node_degrees[neighbors[0] - 1] += 1;
        node_degrees[neighbors[1] - 1] += 1;
    }
    for (i, node) in node_degrees.iter().enumerate() {
        println!("Node {} has a degree of {}", i + 1, node);
    }
}
