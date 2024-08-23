#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    // Add other fields as necessary
}

// Implement any methods for User if needed
impl User {
    // Example method
    pub fn new(id: i32, username: String, email: String) -> Self {
        Self { id, username, email }
    }
}