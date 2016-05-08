use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

type Stats = HashMap<char, u64>;

fn calculate_stats(s: &str) -> Stats {
    let mut stats = Stats::new();
    for c in s.chars() {
        *stats.entry(c).or_insert(0) += 1;
    }
    stats
}

fn entropy(s: &str) -> f64 {
    let stats = calculate_stats(s);
    -stats.iter()
          .fold(0f64, |acc, (_, &i)| {
              let t = i as f64 / s.len() as f64;
              acc + t * t.log2()
          })
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{:.9}", entropy(&line.unwrap()));
    }
}
