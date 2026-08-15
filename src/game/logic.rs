use std::cmp::Ordering;

use rand::Rng;

pub fn generate_secret_number() -> u8 {
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    secret_number
}

pub fn compare(guess: u8, secret_number: u8) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        }

        Ordering::Equal => {
            println!("You win!");
            true
        }

        Ordering::Greater => {
            println!("Too big!");
            false
        }
    }
}
