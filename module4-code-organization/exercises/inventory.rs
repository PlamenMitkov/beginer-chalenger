use crate::product::Product;
use std::collections::HashMap;

pub struct Inventory {
    stock: HashMap<u32, (Product, u32)>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            stock: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product, quantity: u32) {
        self.stock.insert(product.id, (product, quantity));
    }

    pub fn remove_product(&mut self, product_id: u32, quantity: u32) -> bool {
        if let Some((_, current_quantity)) = self.stock.get_mut(&product_id) {
            if *current_quantity >= quantity {
                *current_quantity -= quantity;
                return true;
            }
        }
        false
    }

    pub fn check_stock(&self, product_id: u32) -> Option<u32> {
        self.stock.get(&product_id).map(|(_, qty)| *qty)
    }
}