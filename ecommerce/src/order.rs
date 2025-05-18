use crate::product::Product;
use crate::user::User;

#[derive(Debug)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug)]
pub struct Order {
    pub id: u32,
    pub user: User,
    pub products: Vec<(Product, u32)>, // (Product, quantity)
    pub status: OrderStatus,
}

impl Order {
    pub fn new(id: u32, user: User) -> Self {
        Order {
            id,
            user,
            products: Vec::new(),
            status: OrderStatus::Pending,
        }
    }

    pub fn add_product(&mut self, product: Product, quantity: u32) {
        self.products.push((product, quantity));
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status;
    }

    pub fn display(&self) {
        println!("Order ID: {}", self.id);
        println!("Status: {:?}", self.status);
        println!("Customer:");
        self.user.display();
        println!("Products:");
        for (product, quantity) in &self.products {
            println!("- {}x {}", quantity, product.name);
        }
    }
}
