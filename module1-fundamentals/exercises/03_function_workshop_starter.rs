/// A module containing mathematical and conversion utilities
#[allow(dead_code)]

/// Adds two 32-bit integers
/// 
/// # Arguments
/// * `a` - First integer
/// * `b` - Second integer
/// 
/// # Returns
/// Sum of the two integers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Calculates the area of a rectangle
/// 
/// # Arguments
/// * `width` - Width of the rectangle
/// * `height` - Height of the rectangle
/// 
/// # Returns
/// Area of the rectangle in square units
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    if width < 0.0 || height < 0.0 {
        panic!("Rectangle dimensions cannot be negative");
    }
    width * height
}

/// Checks if a given number is prime
/// 
/// # Arguments
/// * `number` - The number to check
/// 
/// # Returns
/// `true` if the number is prime, `false` otherwise
fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }
    
    let sqrt = (number as f64).sqrt() as u32;
    for i in (3..=sqrt).step_by(2) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

/// Converts temperature from Fahrenheit to Celsius
/// 
/// # Arguments
/// * `fahrenheit` - Temperature in Fahrenheit
/// 
/// # Returns
/// Temperature in Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    // Addition examples
    let sum1 = add(10, 20);
    let sum2 = add(10, 25);
    
    // Rectangle area calculations
    let area1 = calculate_rectangle_area(5.0, 10.0);
    let area2 = calculate_rectangle_area(3.0, 7.0);
    
    // Prime number testing
    let prime_check1 = is_prime(7);
    let prime_check2 = is_prime(12);
    
    // Temperature conversions
    let celsius1 = fahrenheit_to_celsius(98.6);
    let celsius2 = fahrenheit_to_celsius(32.0);
    
    // Output results
    println!("Addition Results:");
    println!("---------------");
    println!("Sum of 10 and 20 is: {}", sum1);
    println!("Sum of 10 and 25 is: {}", sum2);
    println!();

    println!("Rectangle Area Results:");
    println!("---------------------");
    println!("Area of rectangle (5x10): {:.2} square units", area1);
    println!("Area of rectangle (3x7): {:.2} square units", area2);
    println!();
    
    println!("Prime Number Check Results:");
    println!("-------------------------");
    println!("Is 7 a prime number? {}", prime_check1);
    println!("Is 12 a prime number? {}", prime_check2);
    println!();

    println!("Temperature Conversion Results:");
    println!("----------------------------");
    println!("98.6°F = {:.1}°C", celsius1);
    println!("32.0°F = {:.1}°C", celsius2);
}