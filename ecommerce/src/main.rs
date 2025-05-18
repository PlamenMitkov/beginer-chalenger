mod product;
mod user;
mod order;
mod inventory;

use product::Product;
use user::User;
use order::{Order, OrderStatus};
use inventory::Inventory;

fn main() {
    // Initialize the inventory system
    let mut inventory = Inventory::new();

    // Create example products
    let laptop = Product::new(
        1,
        String::from("Laptop"),
        999.99,
        String::from("High-performance laptop with 16GB RAM"),
    );
    let mouse = Product::new(
        2,
        String::from("Wireless Mouse"),
        29.99,
        String::from("Ergonomic wireless mouse"),
    );

    // Add products to inventory
    inventory.add_stock(laptop.id, 5);
    inventory.add_stock(mouse.id, 10);

    // Create a user
    let user = User::new(
        1,
        String::from("John Doe"),
        String::from("john.doe@example.com"),
        String::from("123 Main St, City, Country"),
    );

    // Create and process an order
    let mut order = Order::new(1, user.clone());
    order.add_product(laptop.clone(), 1);
    order.add_product(mouse.clone(), 2);

    // Display initial state
    println!("\n=== Initial Inventory ===");
    inventory.display_stock();

    // Process the order
    if inventory.remove_stock(laptop.id, 1) && inventory.remove_stock(mouse.id, 2) {
        order.update_status(OrderStatus::Processing);
        println!("\n=== Order Processed Successfully ===");
        order.display();
    } else {
        println!("\n=== Order Processing Failed: Insufficient Stock ===");
    }

    // Show updated inventory
    println!("\n=== Updated Inventory ===");
    inventory.display_stock();

    // Try to order more than available (using the product IDs)
    println!("\n=== Attempting to Order Out-of-Stock Items ===");
    let mut large_order = Order::new(2, user);
    large_order.add_product(laptop.clone(), 10); // More than we have in stock

    if inventory.remove_stock(1, 10) { // Using product ID directly
        large_order.update_status(OrderStatus::Processing);
        println!("Order processed successfully");
    } else {
        println!("Order failed: Insufficient stock");
        large_order.update_status(OrderStatus::Cancelled);
    }

    println!("\n=== Final Inventory ===");
    inventory.display_stock();
}
