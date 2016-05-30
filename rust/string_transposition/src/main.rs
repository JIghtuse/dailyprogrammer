use std::io;

fn get_number<T>(stdin: &io::Stdin) -> Option<T>
    where T: std::str::FromStr
{
    let mut s = String::new();
    match stdin.read_line(&mut s) {
        Err(_) => None,
        Ok(_) => s.trim().parse::<T>().ok(),
    }
}

fn read_words() -> Vec<String> {
    let stdin = io::stdin();
    let nwords: usize = get_number(&stdin).unwrap();

    let mut words = vec![];

    for _ in 0..nwords {
        let mut word = String::new();
        stdin.read_line(&mut word).unwrap();
        words.push(word.trim().to_string());
    }
    words
}

fn main() {
    let words = read_words();
    let max_length = words.iter().map(|s| s.len()).max().unwrap();

    for i in 0..max_length {
        for word in &words {
            let c = if i >= word.len() {
                ' '
            } else if let &Some(c) = &word[i..i + 1].chars().next() {
                c
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!("");
    }
}
