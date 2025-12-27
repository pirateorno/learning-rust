use rand::Rng;
use colored::*;

fn main() {
    let stupid_array = ["Sosisocki", "chivavue", "arbuzikes"];

    for word in stupid_array.iter() {
        let random = rand::thread_rng().gen_range(1..=3);
        match random {
            1 => println!("{}", word.blue()),
            2 => println!("{}", word.yellow()),
            _ => println!("{}", word.red())
        }
    }
}