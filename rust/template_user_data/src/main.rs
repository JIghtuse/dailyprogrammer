use std::io::{self, Write};

fn get_line(prompt: &str, stdin: &io::Stdin) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let stdin = io::stdin();
    let name = get_line("Enter your name: ", &stdin);
    let age = get_line("Enter your age: ", &stdin);
    let age = age.parse::<u32>().expect("Invalid input!");
    let username = get_line("Enter your username: ", &stdin);

    print!("your name is {}", name);
    print!(", you are {} age old", age);
    println!(", and your username is {}", username);
}
