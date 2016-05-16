use std::io;

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if a >= b {
        gcd(a % b, b)
    } else {
        gcd(a, b % a)
    }
}

fn get_numbers() -> (u64, u64) {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read number");

    let mut numbers = s.split_whitespace().map(|word| word.parse().unwrap());
    (numbers.next().unwrap(), numbers.next().unwrap())
}

fn main() {
    let numbers = get_numbers();
    println!("{}", gcd(numbers.0, numbers.1));
}
