use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => break, // If the input is not a number, quit the game. ie. 'q'
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // psuedo: Compare the value of 'guess' with 'secret_number'
            Ordering::Less => println!("Too small!"), //psuedo:  If 'guess' is less than 'secret_number': Print "Too small!"
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
