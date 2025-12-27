use std::{io, thread, time};
use std::io::Write;
use colored::*;

fn main() {
    const START: i32 = 1000;
    let mut kaneki = START;
    while kaneki > 6 {
        println!("{}", format!("{kaneki} - 7 = {}", kaneki - 7).red());
        kaneki = kaneki - 7;
        thread::sleep(time::Duration::from_millis(10));
    }
    
    for _ in 1..=5 {
        print!("{}", ".".yellow());
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }
    print!("\n");

    for n in 1..=100 {
        println!("{}", format!("Sosal hui inside {n}").blue());
        thread::sleep(time::Duration::from_millis(10));
    }
}