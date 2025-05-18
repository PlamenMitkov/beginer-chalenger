#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub address: String,
}

impl User {
    pub fn new(id: u32, name: String, email: String, address: String) -> Self {
        User {
            id,
            name,
            email,
            address,
        }
    }

    pub fn display(&self) {
        println!("User: {} (ID: {})", self.name, self.id);
        println!("Email: {}", self.email);
        println!("Address: {}", self.address);
    }
}
