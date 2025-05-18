use std::fmt;

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub address: String,
}

#[derive(Debug)]
pub enum UserError {
    InvalidEmail,
    EmptyName,
    EmptyAddress,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserError::InvalidEmail => write!(f, "Invalid email format"),
            UserError::EmptyName => write!(f, "Name cannot be empty"),
            UserError::EmptyAddress => write!(f, "Address cannot be empty"),
        }
    }
}

impl User {
    /// Creates a new User with validation
    pub fn new(id: u32, name: String, email: String, address: String) -> Result<Self, UserError> {
        // Validate inputs
        if name.trim().is_empty() {
            return Err(UserError::EmptyName);
        }
        if address.trim().is_empty() {
            return Err(UserError::EmptyAddress);
        }
        if !Self::is_valid_email(&email) {
            return Err(UserError::InvalidEmail);
        }

        Ok(User {
            id,
            name: name.trim().to_string(),
            email: email.trim().to_lowercase(),
            address: address.trim().to_string(),
        })
    }

    /// Creates a new User without validation (use with caution)
    pub fn new_unchecked(id: u32, name: String, email: String, address: String) -> Self {
        User {
            id,
            name,
            email: email.to_lowercase(),
            address,
        }
    }

    /// Validates email format (basic check)
    fn is_valid_email(email: &str) -> bool {
        let email = email.trim();
        email.contains('@') && 
        email.split('@').count() == 2 && 
        !email.starts_with('@') && 
        !email.ends_with('@')
    }

    /// Updates the user's address
    pub fn update_address(&mut self, new_address: String) -> Result<(), UserError> {
        if new_address.trim().is_empty() {
            return Err(UserError::EmptyAddress);
        }
        self.address = new_address.trim().to_string();
        Ok(())
    }

    /// Updates the user's email
    pub fn update_email(&mut self, new_email: String) -> Result<(), UserError> {
        if !Self::is_valid_email(&new_email) {
            return Err(UserError::InvalidEmail);
        }
        self.email = new_email.trim().to_lowercase();
        Ok(())
    }

    /// Displays user information in a formatted way
    pub fn display(&self) {
        println!("┌─────────── User Details ───────────┐");
        println!("│ ID: {:<29} │", self.id);
        println!("│ Name: {:<27} │", self.name);
        println!("│ Email: {:<26} │", self.email);
        println!("│ Address: {:<24} │", self.address);
        println!("└────────────────────────────────────┘");
    }

    /// Returns user information as a formatted string
    pub fn to_string(&self) -> String {
        format!("User {} (ID: {}) - Email: {}, Address: {}", 
            self.name, self.id, self.email, self.address)
    }
}
