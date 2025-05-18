#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub address: String,
}

impl User {
    pub fn new(id: u32, name: &str, email: &str, address: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            address: address.to_string(),
        }
    }

    pub fn display(&self) {
        println!("User: {} (ID: {})", self.name, self.id);
        println!("Email: {}", self.email);
        println!("Address: {}", self.address);
    }
}