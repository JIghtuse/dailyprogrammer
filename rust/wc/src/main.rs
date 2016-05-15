use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::env;

struct CountStat {
    lines: usize,
    words: usize,
}

fn get_count_stat<P: AsRef<Path>>(filename: P) -> Option<CountStat> {
    if let Ok(file) = File::open(filename) {
        let mut words = 0usize;
        let mut lines = 0usize;

        let reader = BufReader::new(file);
        for line in reader.lines() {
            words += line.unwrap().split_whitespace().count();
            lines += 1;
        }
        Some(CountStat {
            lines: lines,
            words: words,
        })
    } else {
        None
    }
}

fn main() {
    let mut args = env::args();
    let program_name = args.next();

    if let Some(input_file) = args.next() {
        if let Some(stats) = get_count_stat(&input_file) {
            println!("{} ", &input_file);
            println!("\tlines: {}", stats.lines);
            println!("\twords: {}", stats.words);
        }
    } else {
        println!("usage: {} input_file", program_name.unwrap());
    }
}
