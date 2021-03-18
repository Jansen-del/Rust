// this program generates a random number for the user to guess
#![allow(unused_imports)]

use rand::Rng;
use std::io::stdin;



fn main()
{
    let number = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Enter your guess: ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                    let parsed = buffer.trim_end().parse::<i64>();
                    match parsed {
                        Ok(guess) => {
                            if guess < 1 || guess > 100 {
                                println!("your guess is out of range");
                            } else if guess < number{
                                println!("your guess is too low");
                            } else if guess > number {
                                println!("your guess is too high");
                            } else {println!("good job {} {}", number, guess );
                        break;}
                        }
                        Err(e) => {
                            println!("could not read your input {}", e);
                        }
                    }
            },
            Err(_) => continue,
        }
    }
}

