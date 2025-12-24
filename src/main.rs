use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Salam zaebal");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Input your ebaniy guess: ");

    loop {
        let mut random_guess = String::new();

        io::stdin()
            .read_line(&mut random_guess)
            .expect("Failed to read line");

        let random_guess: u32 = match random_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match random_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small blya"),
            Ordering::Greater => println!("Too big sosunocok"),
            Ordering::Equal => {
                println!("You win nihua sobi");
                break;
            },
        }
    }
} 