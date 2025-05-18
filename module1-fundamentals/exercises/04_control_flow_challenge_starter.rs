use std::io::{self, Write};

/// Represents the available calculator operations
#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exit,
}

impl Operation {
    fn from_u32(value: u32) -> Option<Operation> {
        match value {
            1 => Some(Operation::Add),
            2 => Some(Operation::Subtract),
            3 => Some(Operation::Multiply),
            4 => Some(Operation::Divide),
            5 => Some(Operation::Exit),
            _ => None,
        }
    }
}

/// Prints FizzBuzz sequence for numbers 1 to n
fn print_fizzbuzz(n: u32) {
    println!("=== FizzBuzz Challenge ===");
    for i in 1..=n {
        let output = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => i.to_string(),
        };
        println!("{}", output);
    }
}

/// Reads a number from standard input with type conversion
fn read_number<T>(prompt: &str) -> T 
where 
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
            
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

/// Performs the calculator operation
fn perform_operation(op: Operation, num1: f64, num2: f64) -> Option<f64> {
    match op {
        Operation::Add => Some(num1 + num2),
        Operation::Subtract => Some(num1 - num2),
        Operation::Multiply => Some(num1 * num2),
        Operation::Divide => {
            if num2 == 0.0 {
                println!("Error: Cannot divide by zero.");
                None
            } else {
                Some(num1 / num2)
            }
        },
        Operation::Exit => None,
    }
}

/// Runs the calculator program
fn run_calculator() {
    println!("\n=== Calculator ===");
    
    loop {
        println!("\nChoose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        
        let choice: u32 = read_number("Enter your choice: ");
        
        let operation = match Operation::from_u32(choice) {
            Some(Operation::Exit) => break,
            Some(op) => op,
            None => {
                println!("Invalid option. Please try again.");
                continue;
            }
        };
        
        let num1: f64 = read_number("Enter the first number: ");
        let num2: f64 = read_number("Enter the second number: ");
        
        if let Some(result) = perform_operation(operation, num1, num2) {
            println!("Result: {:.2}", result);
        }
        
        print!("Do you want to perform another calculation? (y/n): ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut again = String::new();
        io::stdin()
            .read_line(&mut again)
            .expect("Failed to read input");
            
        if again.trim().to_lowercase() != "y" {
            break;
        }
    }
    
    println!("Thank you for using the calculator!");
}

fn main() {
    print_fizzbuzz(20);
    run_calculator();
}