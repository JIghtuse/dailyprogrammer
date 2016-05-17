use std::io;

fn get_number() -> u32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn get_prefix(number: u32) -> &'static str {
    match number % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

fn main() {
    let max_number = 100;
    let number = get_number();

    for i in 0..number {
        print!("{}{} ", i, get_prefix(i));
    }
    for i in number + 1..max_number + 1 {
        print!("{}{} ", i, get_prefix(i));
    }
    println!("");
}
