use std::collections::HashMap;
use crate::product::Product;

pub struct Inventory {
    stock: HashMap<u32, u32>, // product_id -> quantity
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            stock: HashMap::new(),
        }
    }

    pub fn add_stock(&mut self, product_id: u32, quantity: u32) {
        *self.stock.entry(product_id).or_insert(0) += quantity;
    }

    pub fn remove_stock(&mut self, product_id: u32, quantity: u32) -> bool {
        if let Some(current_quantity) = self.stock.get_mut(&product_id) {
            if *current_quantity >= quantity {
                *current_quantity -= quantity;
                return true;
            }
        }
        false
    }

    pub fn check_stock(&self, product_id: u32) -> u32 {
        *self.stock.get(&product_id).unwrap_or(&0)
    }

    pub fn display_stock(&self) {
        println!("Current Inventory:");
        for (product_id, quantity) in &self.stock {
            println!("Product ID: {}, Quantity: {}", product_id, quantity);
        }
    }
}
