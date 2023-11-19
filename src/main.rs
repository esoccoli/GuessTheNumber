use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the number guessing game!");
    println!(
        "Your goal will be to guess a randomly generated number in as few guesses as possible."
    );

    println!("Generating a random number between 1 and 100 (inclusive)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
