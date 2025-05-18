// Declare our modules
pub mod product;
pub mod user;
pub mod order;
pub mod inventory;

// Bring commonly used items into scope
use product::Product;
use user::User;
use order::{Order, OrderStatus};
use inventory::Inventory;

fn main() {
    // 1. Create some products
    let laptop = Product::new(1, "Laptop", 999.99, "High performance laptop");
    let mouse = Product::new(2, "Mouse", 25.50, "Wireless mouse");
    
    // 2. Add products to inventory
    let mut inventory = Inventory::new();
    inventory.add_product(laptop.clone(), 10);
    inventory.add_product(mouse.clone(), 50);
    
    // 3. Create a user
    let user = User::new(1, "John Doe", "john@example.com", "123 Main St");
    
    // 4. Create an order
    let mut order = Order::new(1, user, OrderStatus::Processing);
    order.add_product(laptop);
    order.add_product(mouse);
    
    // 5. Print order details
    order.display();
    println!("Inventory status: {:?}", inventory.check_stock(1));
}