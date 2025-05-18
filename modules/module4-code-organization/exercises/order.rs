use crate::product::Product;
use crate::user::User;

#[derive(Debug)]
pub enum OrderStatus {
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

pub struct Order {
    pub id: u32,
    pub user: User,
    pub products: Vec<Product>,
    pub status: OrderStatus,
}

impl Order {
    pub fn new(id: u32, user: User, status: OrderStatus) -> Self {
        Order {
            id,
            user,
            products: Vec::new(),
            status,
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn display(&self) {
        println!("Order #{}", self.id);
        println!("Status: {:?}", self.status);
        println!("Customer:");
        self.user.display();
        println!("Products:");
        for product in &self.products {
            product.display();
            println!("---");
        }
        println!("Total: ${:.2}", self.products.iter().map(|p| p.price).sum::<f64>());
    }
}