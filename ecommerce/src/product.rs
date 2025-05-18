#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub description: String,
}

impl Product {
    pub fn new(id: u32, name: String, price: f64, description: String) -> Self {
        Product {
            id,
            name,
            description,
            price,
        }
    }

    pub fn display(&self) {
        println!("Product: {} (ID: {})", self.name, self.id);
        println!("Price: ${:.2}", self.price);
        println!("Description: {}", self.description);
    }
}
