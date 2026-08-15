use crate::game::logic;
use crate::utils::format;
use crate::game::input;

mod utils;
mod game;

fn main() {
    let secret_number = logic::generate_secret_number();
    
    loop {
        format::line("-=", 16);
        println!("Try to guess the number between 1 and 100");
        format::line("-=", 16);
        let guess = input::get_guess();

        if logic::compare(guess, secret_number) {
            break;
        }
    }
}
