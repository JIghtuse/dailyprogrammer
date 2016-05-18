extern crate rand;

use std::fmt::Debug;
use rand::Rng;

// As described in https://class.coursera.org/algs4partI-010/lecture/28
fn shuffle<T, R: Rng>(data: &mut [T], rng: &mut R) {
    for i in 0..data.len() {
        let r = rng.gen_range(0, i + 1);
        data.swap(i, r);
    }
}

fn print_shuffle_print<T: Debug, R: Rng>(data: &mut [T], rng: &mut R) {
    println!("Before shuffle: {:?}", data);
    shuffle(data, rng);
    println!("After shuffle:  {:?}", data);
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut ints = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut strings = vec!["apple",
                           "blackberry",
                           "cherry",
                           "dragonfruit",
                           "grapefruit",
                           "kumquat",
                           "mango",
                           "nectarine",
                           "persimmon",
                           "raspberry",
                           "raspberry"];

    let mut chars = vec!['a', 'e', 'i', 'o', 'u'];

    print_shuffle_print(&mut ints, &mut rng);
    print_shuffle_print(&mut strings, &mut rng);
    print_shuffle_print(&mut chars, &mut rng);
}
