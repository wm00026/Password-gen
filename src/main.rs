// Basic password generator
// author: wm00026
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};
use std::process;

// A constant for a maximum allowed password length.
const MAX_LENGTH: usize = 1024;

/// Gets input from the user.
fn get_input(prompt: &str) -> Result<String, io::Error> {
    let mut input = String::new();
    print!("{} ", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Asks a user a yes/no question
fn get_yes_no(prompt: &str) -> Result<bool, io::Error> {
    let input = get_input(prompt)?.to_lowercase();
    Ok(input == "y" || input == "yes")
}

/// Generates a password made up of characters drawn from given pools
fn generate_password(pools: &[&[char]], length: usize) -> Vec<char> {
    let mut rng = rand::rng();

    let active_pools: Vec<&[char]> = pools
        .iter()
        .copied()
        .filter(|p| !p.is_empty())
        .collect();

    let all_chars: Vec<char> = active_pools
        .iter().
        flat_map(|p| p.iter().copied())
        .collect();

    let mut password: Vec<char> = active_pools
        .iter()
        .take(length)
        .map(|p| p[rng.random_range(0..p.len())])
        .collect();

    while password.len() < length {
        let idx = rng.random_range(0..all_chars.len());
        password.push(all_chars[idx]);
    }

    password.shuffle(&mut rng);
    
    password

}

fn main() -> Result<(), io::Error> {
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "!?$#@";

    println!("Which character types do you want? (Enter Y or N for each):");
    let use_lowercase = get_yes_no("Lowercase letters? (a-z)")?;
    let use_uppercase = get_yes_no("Uppercase letters? (A-Z)")?;
    let use_numbers = get_yes_no("Numbers? (0-9)?")?;
    let use_symbols = get_yes_no("Symbols (!?$#@)?")?;

    let lowercase_chars: Vec<char> = LOWERCASE.chars().collect();
    let uppercase_chars: Vec<char> = UPPERCASE.chars().collect();
    let number_chars: Vec<char> = NUMBERS.chars().collect();
    let symbols_chars: Vec<char> = SYMBOLS.chars().collect();

    let mut pools: Vec<&[char]> = Vec::new();
    if use_lowercase { pools.push(&lowercase_chars); }
    if use_uppercase { pools.push(&uppercase_chars); }
    if use_numbers   { pools.push(&number_chars); }
    if use_symbols   { pools.push(&symbols_chars); }

    if pools.is_empty() {
        eprintln!("No character types selected, exiting.");
        process::exit(1);
    }


    let length: usize = match get_input("Enter the desired password length:")?.parse() {
        Ok(n) if n > 0 && n <= MAX_LENGTH => n,
        Ok(_) => {
            eprintln!("Please enter a length between 1 and {}", MAX_LENGTH);
            process::exit(1);
        }
        Err(_) => {
            eprintln!("Please enter a valid number greater than 0.");
            process::exit(1);
        }
    };

    if length < pools.len() {
        eprintln!(
            "Warning: Passwor length ({}) is shorter than the selected \
            character types ({}); not all selected types may appear",
            length,
            pools.len()
        );
    }

    let password = generate_password(&pools, length);
    println!("Password: {:?}", password);

    Ok(())
}
