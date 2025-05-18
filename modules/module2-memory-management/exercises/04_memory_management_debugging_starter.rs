/// Memory management debugging exercises demonstrating common Rust ownership patterns

// Problem 1: Fix ownership errors
fn problem1() {
    println!("\n=== Problem 1: Ownership ===");
    
    // 1.1: Fix the double-move error
    let data = vec![1, 2, 3];
    let x = data.clone(); // Create a clone for x
    let y = data;         // Move original data to y
    println!("Vectors x: {:?}, y: {:?}", x, y);

    // 1.2: Fix the ownership issue with the function
    let name = String::from("Rust");
    print_data(&name);    // Pass reference instead of clone
    println!("My name is {}", name);
}

fn print_data(data: &str) {    // Accept string slice instead of owned String
    println!("Data: {}", data);
}

// Problem 2: Fix borrowing conflicts
fn problem2() {
    println!("\n=== Problem 2: Borrowing ===");
    
    // 2.1: Fix the mutable/immutable borrow conflict
    let mut numbers = vec![1, 2, 3];
    let first = numbers[0];    // Copy the value instead of borrowing
    numbers.push(4);
    println!("First element is: {}", first);

    // 2.2: Fix the multiple mutable borrows
    let mut data = String::from("Hello");
    {
        let ref1 = &mut data;
        ref1.push_str(", ");   // Modify string in place
    } // ref1 goes out of scope here
    {
        let ref2 = &mut data;
        ref2.push_str("Rust!"); // Modify string in place
    }
    println!("Data: {}", data);
}

// Problem 3: Fix dangling references
fn problem3() {
    println!("\n=== Problem 3: Dangling References ===");
    
    // 3.1: Fix the dangling reference returned by the function
    let result = get_string();
    println!("Result: {}", result);

    // 3.2: Fix the issue with references outliving the data
    let data = vec![1, 2, 3];    // Move declaration outside the block
    let reference = &data;        // Reference the data that lives long enough
    println!("Reference: {:?}", reference);
}

fn get_string() -> String {    // Return owned String
    String::from("I am no longer a dangling reference")
}

// Problem 4: Fix lifetime problems
fn problem4() {
    println!("\n=== Problem 4: Lifetimes ===");
    
    let string1 = String::from("long string is long");
    let string2 = String::from("short");  // Move string2 to outer scope
    let result = longest(string1.as_str(), string2.as_str());
    println!("Longest string: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Problem 5: Optimize unnecessary cloning
fn problem5() {
    println!("\n=== Problem 5: Optimization ===");
    
    let original = String::from("Rust Programming");
    let len = calculate_length(&original);    // Pass reference

    let names = vec![                         // Initialize vector directly
        String::from("Alice"),
        String::from("Bob")
    ];

    for name in &names {                      // Iterate over references
        print_string(name);                   // Pass string slice
    }

    println!("Original is still: {}", original);
    println!("Length was: {}", len);
    println!("Names: {:?}", names);
}

fn calculate_length(s: &str) -> usize {       // Accept string slice
    s.len()
}

fn print_string(s: &str) {                    // Accept string slice
    println!("{}", s);
}

fn main() {
    println!("Running memory management debugging exercises...");
    
    problem1();
    problem2();
    problem3();
    problem4();
    problem5();
    
    println!("\nAll problems completed successfully!");
}