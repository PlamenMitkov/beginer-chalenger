fn main() {
    // TODO: 1. Declare an immutable integer variable
    let mut my_integer: i32 = 42;

    // TODO: 2. Declare a mutable float variable and modify it later
    let mut y: f64 = 3.14;
    let original_y = y;

    // TODO: Modify the float value
    y = 6.28;

    // TODO: 3. Declare a boolean variable using type inference
    let flag = true;
    
    // TODO: 4. Declare a character variable with explicit type annotation
    let c: char = 'R';

    // TODO: 5. Perform arithmetic operations with the numeric variables
    let addition = my_integer as f64 + original_y;
    let multiplication = my_integer as f64 * original_y;
    
    // TODO: 6. Print all variables and calculation results with appropriate labels
    println!("Integer value: {}", my_integer);
    println!("Original float value: {}", original_y);
    println!("Modified float value: {}", y);
    println!("Boolean value: {}", flag);
    println!("Character value: {}", c);
    println!("Addition result: {:.2}", addition);
    println!("Multiplication result: {:.2}", multiplication);
}