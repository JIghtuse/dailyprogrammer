use std::io::prelude::*;
use std::io;

fn eat_line(stdin: &io::Stdin) {
    let mut s = String::new();
    stdin.read_line(&mut s);
}

fn indent_input(stdin: &io::Stdin, indentation: &str) {
    let mut level = 0;
    let stdin = stdin.lock();
    for line in stdin.lines() {
        let line = line.unwrap();

        let first_word = line.split_whitespace().next();

        if first_word == Some("NEXT") || first_word == Some("ENDIF") {
            level -= 1;
        }

        for _ in 0..level {
            print!("{}", indentation);
        }
        println!("{}", line.trim());

        if first_word == Some("FOR") || first_word == Some("IF") {
            level += 1;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    eat_line(&stdin); // we don't care how many lines we have

    let mut indentation = String::new();
    match stdin.read_line(&mut indentation) {
        Ok(_) => indent_input(&stdin, &indentation[..indentation.len() - 1]),
        Err(_) => println!("Expected indent characters on second line"),
    }
}
