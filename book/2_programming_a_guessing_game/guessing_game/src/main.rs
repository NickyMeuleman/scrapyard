// the program will generate a random integer between 1 and 100.
// It will then prompt the player to enter a guess. After a guess is entered,
// the program will indicate whether the guess is too low or too high.
// If the guess is correct, the game will print a congratulatory message and exit.
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a positive integer.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match secret_number.cmp(&guess) {
            Ordering::Less => println!("The secret number is smaller!"),
            Ordering::Greater => println!("The secret number is bigger!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
