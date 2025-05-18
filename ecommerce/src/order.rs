use std::fmt;
use crate::product::Product;
use crate::user::User;

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "🕒 Pending"),
            OrderStatus::Processing => write!(f, "⚙️ Processing"),
            OrderStatus::Shipped => write!(f, "🚚 Shipped"),
            OrderStatus::Delivered => write!(f, "✅ Delivered"),
            OrderStatus::Cancelled => write!(f, "❌ Cancelled"),
        }
    }
}

#[derive(Debug)]
pub struct Order {
    pub id: u32,
    pub user: User,
    pub products: Vec<(Product, u32)>, // (Product, quantity)
    pub status: OrderStatus,
    pub total: f64,
}

#[derive(Debug)]
pub enum OrderError {
    InvalidQuantity,
    ProductNotFound,
    InvalidStatus,
    EmptyOrder,
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderError::InvalidQuantity => write!(f, "Invalid quantity specified"),
            OrderError::ProductNotFound => write!(f, "Product not found in order"),
            OrderError::InvalidStatus => write!(f, "Invalid status transition"),
            OrderError::EmptyOrder => write!(f, "Cannot process empty order"),
        }
    }
}

impl Order {
    pub fn new(id: u32, user: User) -> Self {
        Order {
            id,
            user,
            products: Vec::new(),
            status: OrderStatus::Pending,
            total: 0.0,
        }
    }

    pub fn add_product(&mut self, product: Product, quantity: u32) -> Result<(), OrderError> {
        if quantity == 0 {
            return Err(OrderError::InvalidQuantity);
        }
        
        // Update total
        self.total += product.price * quantity as f64;
        self.products.push((product, quantity));
        Ok(())
    }

    pub fn remove_product(&mut self, product_id: u32) -> Result<(), OrderError> {
        if let Some(index) = self.products.iter().position(|(p, _)| p.id == product_id) {
            let (product, quantity) = &self.products[index];
            self.total -= product.price * *quantity as f64;
            self.products.remove(index);
            Ok(())
        } else {
            Err(OrderError::ProductNotFound)
        }
    }

    pub fn update_status(&mut self, status: OrderStatus) -> Result<(), OrderError> {
        // Validate status transition
        match (&self.status, &status) {
            (OrderStatus::Cancelled, _) => {
                return Err(OrderError::InvalidStatus)
            }
            (OrderStatus::Delivered, _) => {
                return Err(OrderError::InvalidStatus)
            }
            _ => {}
        }

        self.status = status;
        Ok(())
    }

    pub fn calculate_total(&self) -> f64 {
        self.total
    }

    pub fn is_empty(&self) -> bool {
        self.products.is_empty()
    }

    pub fn display(&self) {
        println!("┌─────────── Order Details ────────────┐");
        println!("│ Order ID: {:<24} │", self.id);
        println!("│ Status: {:<25} │", self.status);
        println!("└────────────────────────────────────┘");
        
        println!("Customer Information:");
        self.user.display();
        
        println!("Products:");
        println!("┌─────────────────────────────────────────────┐");
        for (product, quantity) in &self.products {
            println!("│ {}x {} (${:.2} each)", 
                quantity, product.name, product.price);
        }
        println!("└─────────────────────────────────────────────┘");
        println!("Total: ${:.2}", self.total);
    }

    pub fn get_order_summary(&self) -> String {
        format!("Order #{} - {} - {} items - Total: ${:.2}", 
            self.id, 
            self.status, 
            self.products.len(),
            self.total)
    }
}
