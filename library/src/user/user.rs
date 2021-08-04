#[derive(Debug)]
pub struct User {
    id: [u8; 4],
    username: String,
    password: String,
}

impl User {
    
    pub fn new(id: [u8; 4], username: String, password: String) -> Self {
        User {
            id,
            username,
            password,
        }
    }
}