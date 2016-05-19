#![feature(iter_arith)]

#[macro_use]
extern crate quick_error;

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

quick_error! {
    #[derive(Debug)]
    pub enum DataError {
        Io(err: io::Error) {
            from()
        }
        Format(err: std::num::ParseFloatError) {
            from()
        }
    }
}

type Result<T> = std::result::Result<T, DataError>;


fn get_data<P: AsRef<Path>>(path: P) -> Result<Vec<f32>> {
    let file = try!(File::open(path));
    let reader = BufReader::new(file);
    let mut v = vec![];
    for line in reader.lines() {
        let line = try!(line);
        let sample: f32 = try!(line.parse());
        v.push(sample);
    }
    Ok(v)
}

fn mean(data: &[f32]) -> Option<f32> {
    if data.is_empty() {
        None
    } else {
        let sum : f32 = data.iter().sum();
        Some(sum / data.len() as f32)
    }
}

fn variance(data: &[f32]) -> Option<f32> {
    if data.is_empty() {
        None
    } else {
        let mean = mean(data).unwrap();
        let mut sum = 0f32;
        for &x in data {
            sum += (x - mean).powi(2);
        }
        Some(sum / (data.len() - 1) as f32)
    }
}

fn standard_deviation(data: &[f32]) -> Option<f32> {
    if data.is_empty() {
        None
    } else {
        let variance = variance(data).unwrap();
        Some(variance.sqrt())
    }
}

fn main() {
    if let Ok(data) = get_data("input.txt") {
        println!("{:?}", mean(&data));
        println!("{:?}", variance(&data));
        println!("{:?}", standard_deviation(&data));
    }
}
