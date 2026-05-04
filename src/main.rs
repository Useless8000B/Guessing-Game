use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading!");

        let guess: i32 = guess.trim().parse().expect("Type a number!");
        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => println!("You win!"),
            Ordering::Greater => {
                println!("Too big!");
                break;
            },
        }
    }
}
