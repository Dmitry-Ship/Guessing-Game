use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Read};

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number: {}", secret_number);

    loop {
        println!("Please input your guess");

        // Use a fixed buffer of 16 bytes instead of a growable String
        let mut buffer = [0u8; 16];

        // Read up to buffer size
        match io::stdin().read(&mut buffer) {
            Ok(n) => {
                if n == buffer.len() {
                    // Buffer was filled, likely more input waiting
                    println!("Input too long! Please enter a number with max 9 digits.");
                    // Clear the rest of the line
                    let mut rest = String::new();
                    io::stdin().read_line(&mut rest).unwrap_or(0);
                    continue;
                }

                // Convert buffer to string, only using bytes read
                let input = String::from_utf8_lossy(&buffer[..n]);

                let guess: u32 = match input.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    }
                };
                println!("You guessed {}", guess);

                match guess.cmp(&secret_number) {
                    Ordering::Greater => println!("Too big"),
                    Ordering::Less => println!("Too small"),
                    Ordering::Equal => {
                        println!("You win!");
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                continue;
            }
        }
    }
}
