use std::io;

#[derive(Eq, Hash, PartialEq)]
enum Operation {
    Reverse,
    Capitalize,
}

fn get_number<T>(stdin: &io::Stdin) -> Option<T>
    where T: std::str::FromStr
{
    let mut s = String::new();
    match stdin.read_line(&mut s) {
        Err(_) => None,
        Ok(_) => s.trim().parse::<T>().ok(),
    }
}

fn read_input_line(stdin: &io::Stdin) -> (Operation, String, String) {
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let mut words = s.split_whitespace();
    let op = match words.next() {
        Some("0") => Operation::Reverse,
        Some("1") => Operation::Capitalize,
        _ => unimplemented!(),
    };
    let input = words.next().unwrap().to_string();
    let actual = words.next().unwrap().to_string();
    (op, input, actual)
}

const MISMATCH_MESSAGE: &'static str = "Mismatch! Bad test data";
const GOOD_TEST_MESSAGE: &'static str = "Good test data";

fn main() {
    let stdin = io::stdin();
    let ntests: usize = get_number(&stdin).unwrap();
    for _ in 0..ntests {
        let (op, input, actual) = read_input_line(&stdin);
        let msg = match op {
            Operation::Reverse => {
                if input.chars().rev().zip(actual.chars()).all(|c| c.0 == c.1) {
                    GOOD_TEST_MESSAGE
                } else {
                    MISMATCH_MESSAGE
                }
            }
            Operation::Capitalize => {
                if input.to_uppercase() == actual {
                    GOOD_TEST_MESSAGE
                } else {
                    MISMATCH_MESSAGE
                }
            }
        };
        println!("{}", msg);
    }
}
