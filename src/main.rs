// Basic password generator
// author: wm00026
use rand::Rng;
use std::io;

/// A function to get the input from the user
/// param: prompt: the prompt for what the user should type
/// returns a String of the input
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().
        read_line(&mut input).
        expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    // Pool of possible characters

    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!?$#@";

    println!("Which character types do you want? (Enter Y or N for each:");

    // Gets the input from the user about what characters they want to use
    let use_lowercase = get_input("Lowercase letters (a-z)?").to_lowercase() == "y";
    let use_uppercase = get_input("Uppercase letters (A-Z)?").to_lowercase() == "y";
    let use_numbers = get_input("Numbers (0-9)?").to_lowercase() == "y";
    let use_symbols = get_input("Symbols (!?@$#)?").to_lowercase() == "y";

    // Places the letters into the pool
    let mut pool = String::new();
    if use_lowercase { pool.push_str(lowercase); }
    if use_uppercase { pool.push_str(uppercase); }
    if use_numbers { pool.push_str(numbers); }
    if use_symbols { pool.push_str(symbols); }

    if pool.is_empty() {
        println!("No character types selected, exiting.");
        return;
    }

    let chars: Vec<char> = pool.chars().collect();

    // Asks for the password length
    let length: usize = get_input("Enter the desired password length:").
        parse().
        expect("Please enter a valid number!");

    if length == 0 {
        println!("Password length must be greater than 0. Exiting.");
        return;
    }

    let mut rng = rand::rng();
    
    // Creates the password
    let password: String = (0..length).
        map(|_| {
            let idx = rng.random_range(0..chars.len());
            chars[idx]
        }).
    collect();

    println!("Password: {}", password);
}
