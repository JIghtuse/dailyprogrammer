use std::convert::From;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::num;

#[derive(Debug)]
enum DataError {
    Io,
    Parse,
}

type Result<T> = std::result::Result<T, DataError>;

impl From<io::Error> for DataError {
    fn from(_: io::Error) -> Self {
        DataError::Io
    }
}

impl From<num::ParseIntError> for DataError {
    fn from(_: num::ParseIntError) -> Self {
        DataError::Parse
    }
}

#[derive(Debug)]
struct UserData {
    name: String,
    age: u32,
    username: String,
}

fn get_line(prompt: &str, stdin: &io::Stdin) -> Result<String> {
    print!("{}", prompt);
    try!(io::stdout().flush());
    let mut s = String::new();
    try!(stdin.read_line(&mut s));
    Ok(s.trim().to_string())
}

fn get_user_data(stdin: &io::Stdin) -> Result<UserData> {
    let name = try!(get_line("Enter your name: ", &stdin));
    let age = try!(get_line("Enter your age: ", &stdin));
    let age = try!(age.parse::<u32>());
    let username = try!(get_line("Enter your username: ", &stdin));
    Ok(UserData {
        name: name,
        age: age,
        username: username,
    })
}

fn save_user_data(filename: &str, data: &UserData) -> Result<()> {
    let mut buffer = try!(File::create(filename));
    try!(buffer.write_fmt(format_args!("{:?}", data)));
    Ok(())
}

fn main() {
    let stdin = io::stdin();

    match get_user_data(&stdin) {
        Ok(user_data) => {
            print!("your name is {}", user_data.name);
            print!(", you are {} age old", user_data.age);
            println!(", and your username is {}", user_data.username);
            save_user_data("user_data.txt", &user_data);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
