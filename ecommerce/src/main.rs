mod product;
mod user;
mod order;
mod inventory;

use product::Product;
use user::User;
use order::{Order, OrderStatus};
use inventory::Inventory;

fn main() {
    println!("🏪 E-Commerce System Demo");
    println!("========================\n");

    // Initialize the inventory system
    let mut inventory = Inventory::new();

    // Create example products
    let laptop = Product::new(
        1,
        String::from("MacBook Pro"),
        1299.99,
        String::from("Latest model with M1 chip and 16GB RAM"),
    );
    let mouse = Product::new(
        2,
        String::from("Magic Mouse"),
        79.99,
        String::from("Wireless Magic Mouse 2"),
    );
    let keyboard = Product::new(
        3,
        String::from("Magic Keyboard"),
        99.99,
        String::from("Wireless keyboard with numeric keypad"),
    );

    // Add products to inventory
    println!("📦 Stocking Inventory...");
    inventory.add_stock(laptop.id, 5);
    inventory.add_stock(mouse.id, 10);
    inventory.add_stock(keyboard.id, 8);

    // Display initial inventory
    println!("\n📋 Initial Inventory Status:");
    inventory.display_stock();

    // Create a user
    println!("\n👤 Creating New User...");
    let user = User::new(
        1,
        String::from("John Doe"),
        String::from("john.doe@example.com"),
        String::from("123 Main St, City, Country"),
    ).expect("Failed to create user");

    // Display user information
    println!("\n📝 User Information:");
    user.display();

    // Create and process first order
    println!("\n🛒 Processing First Order...");
    let mut order1 = Order::new(1, user.clone());
    order1.add_product(laptop.clone(), 1).expect("Failed to add product");
    order1.add_product(mouse.clone(), 2).expect("Failed to add product");

    // Process the order
    println!("\n📦 Order Details:");
    order1.display();

    if inventory.remove_stock(laptop.id, 1) && inventory.remove_stock(mouse.id, 2) {
        order1.update_status(OrderStatus::Processing).expect("Failed to update status");
        println!("\n✅ Order Processed Successfully");
        println!("Order Summary: {}", order1.get_order_summary());
    } else {
        order1.update_status(OrderStatus::Cancelled).expect("Failed to update status");
        println!("\n❌ Order Processing Failed: Insufficient Stock");
    }

    // Show updated inventory
    println!("\n📋 Updated Inventory Status:");
    inventory.display_stock();

    // Try to order more than available
    println!("\n🛒 Attempting Large Order (Should Fail)...");
    let mut order2 = Order::new(2, user.clone());
    order2.add_product(laptop.clone(), 10).expect("Failed to add product");
    
    println!("\n📦 Large Order Details:");
    order2.display();

    if inventory.remove_stock(laptop.id, 10) {
        order2.update_status(OrderStatus::Processing).expect("Failed to update status");
        println!("\n✅ Large Order Processed");
    } else {
        order2.update_status(OrderStatus::Cancelled).expect("Failed to update status");
        println!("\n❌ Large Order Failed: Insufficient Stock");
    }

    // Final inventory check
    println!("\n📋 Final Inventory Status:");
    inventory.display_stock();

    // Demonstrate order status progression
    println!("\n🔄 Demonstrating Order Status Progression...");
    let mut order3 = Order::new(3, user);
    order3.add_product(keyboard.clone(), 1).expect("Failed to add product");
    
    println!("\nInitial Status: {}", order3.get_order_summary());
    
    // Process order
    order3.update_status(OrderStatus::Processing).expect("Failed to update status");
    println!("After Processing: {}", order3.get_order_summary());
    
    // Ship order
    order3.update_status(OrderStatus::Shipped).expect("Failed to update status");
    println!("After Shipping: {}", order3.get_order_summary());
    
    // Deliver order
    order3.update_status(OrderStatus::Delivered).expect("Failed to update status");
    println!("After Delivery: {}", order3.get_order_summary());
    
    println!("\n✨ Demo Completed Successfully!");
}
