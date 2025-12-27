use std::io;

fn main() {
    println!("Welcome to my very good calculator");

    loop {
        println!("1 is +");
        println!("2 is -");
        println!("3 is /");
        println!("4 is *");
        println!("0 to exit");
        

        let choise = get_input("Enter what you want to do: ");

        if choise == 0 {
            println!("Goodbye pidor!");
            break;
        }

        let first = get_input("Enter first number: ");
        let second = get_input("Enter second number: ");

        let result = match choise {
            1 => first + second,
            2 => first - second,
            3 => {
                if second == 0 {
                    println!("Bro you need to learn some math stupid black monkey");
                    continue;
                }
                first / second
            },
            4 => first * second,
            _ => {
                println!("Bro what are you typing");
                continue;
            }
        };

        println!("Result: {result}");
    }
}

fn get_input(question: &str) -> i32 {
    loop {
        println!("{}", question);

        let mut shit = String::new();
        io::stdin()
            .read_line(&mut shit)
            .expect("Failed to read line");

        match shit.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Enter valid number nigga"),
        }
    }
}