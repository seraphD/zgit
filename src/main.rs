use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess Number!");
    println!("input your number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Numebr: {secret_number}");

    let mut guess = String::new();

    loop {
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Right!!");
                break;
            },
            Ordering::Greater => println!("Big"),
            Ordering::Less => println!("Small"),
        }
    }
}
