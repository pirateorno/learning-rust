use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Huesos program");

    loop {
        let first_random = rand::thread_rng().gen_range(1..=100);

        let second_random = rand::thread_rng().gen_range(1..=100);

        match first_random.cmp(&second_random) {
            Ordering::Greater => println!("{first_random} - {second_random} = {}", first_random - second_random),
            Ordering::Equal => {
                println!("{first_random} - {second_random} = {}", first_random - second_random);
                println!("sosi huy");
                break;
            }
            Ordering::Less => println!("{second_random} - {first_random} = {}",  second_random - first_random),
        };
    }
}