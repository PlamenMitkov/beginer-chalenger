use std::io;
use chrono::Local;

fn main() {
    // TODO: 1. Prompt the user for their name
     println!("What is your name?");
    // TODO: 2. Read the user's input
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let trimmed_name = name.trim();


    // TODO: 3. Print a personalized greeting
    println!("Hello, {}! Welcome to the Rust Bootcamp!", trimmed_name);
    // BONUS: Print the current date
    // Hint: You can use the chrono crate for this
    let current_date = Local::now().format("%Y-%m-%d").to_string();
    println!("Today is {}.", current_date);
}