// Basic password generator
// author: wm00026
use std::io::{self, Write};
use rand::Rng;

/// Gets input from the user.
/// # Arguments
/// * `prompt` - The prompt displayed to the user
/// # Returns
/// A `Result` containing the trimmed input string, or an `io::Error`
fn get_input(prompt: &str) -> Result<String, io::Error> {
    let mut input = String::new();
    print!("{} ", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Generates the password for the user
/// # Arguments
/// * `chars` - The characters within the password
/// * `length` - The length of the password
fn generate_password(chars: &[char], length: usize) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..chars.len()); 
            chars[idx]
        })
        .collect()
}

fn main() -> Result<(), io::Error> {
    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &str = "0123456789";
    const SYMBOLS: &str = "!?$#@";

    println!("Which character types do you want? (Enter Y or N for each):");

    let use_lowercase = get_input("Lowercase letters (a-z)?")?.to_lowercase() == "y";
    let use_uppercase = get_input("Uppercase letters (A-Z)?")?.to_lowercase() == "y";
    let use_numbers   = get_input("Numbers (0-9)?")?.to_lowercase() == "y";
    let use_symbols   = get_input("Symbols (!?$#@)?")?.to_lowercase() == "y";

    let mut pool = String::new();
    if use_lowercase { pool.push_str(LOWERCASE); }
    if use_uppercase { pool.push_str(UPPERCASE); }
    if use_numbers   { pool.push_str(NUMBERS); }
    if use_symbols   { pool.push_str(SYMBOLS); }

    if pool.is_empty() {
        eprintln!("No character types selected, exiting.");
        return Ok(());
    }

    let chars: Vec<char> = pool.chars().collect();

    let length: usize = match get_input("Enter the desired password length:")?.parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("Please enter a valid number greater than 0.");
            return Ok(());
        }
    };

    let password = generate_password(&chars, length);
    println!("Password: {}", password);

    Ok(())
}
