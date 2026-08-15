use std::io;

pub fn get_guess() -> u8 {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading guess!");

        match guess.trim().parse() {
            Ok(num) => {
                if num > 100 || num < 1 {
                    println!("The number will be between 1 and 100");
                    continue;
                } else {
                    return num;
                }
            }

            Err(_) => {
                println!("Error parsing value, try again");
                continue;
            }
        };
    }
}
