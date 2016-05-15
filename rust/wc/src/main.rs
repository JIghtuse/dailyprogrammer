use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::env;

struct CountStat {
    lines: usize,
}

fn get_count_stat<P: AsRef<Path>>(filename: P) -> Option<CountStat> {
    if let Ok(file) = File::open(filename) {
        let reader = BufReader::new(file);
        let lines = reader.lines().count();
        Some(CountStat { lines: lines })
    } else {
        None
    }
}

fn main() {
    let mut args = env::args();
    let program_name = args.next();

    if let Some(input_file) = args.next() {
        if let Some(stats) = get_count_stat(&input_file) {
            println!("Number of lines in {}: {}", &input_file, stats.lines);
        }
    } else {
        println!("usage: {} input_file", program_name.unwrap());
    }
}
