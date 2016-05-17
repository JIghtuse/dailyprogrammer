use std::io;

fn get_number() -> u32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn get_prefix(number: u32) -> &'static str {
    match number % 100 {
        11 | 12 | 13 => "th",
        n => {
            match n % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            }
        }
    }
}

fn print_placements(min_place: u32, max_place: u32, our_dog_place: u32) {
    let print_place = |i: u32| print!("{}{} ", i, get_prefix(i));

    for i in min_place..our_dog_place {
        print_place(i);
    }
    for i in our_dog_place + 1..max_place + 1 {
        print_place(i);
    }
    println!("");
}

fn main() {
    let min_place = 1;
    let max_place = 100;
    let our_dog_place = get_number();

    print_placements(min_place, max_place, our_dog_place);
}
