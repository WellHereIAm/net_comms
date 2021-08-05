#[derive(Debug)]
pub struct User {
    id: usize,
    username: String,
    password: String,
}

impl User {
    
    pub fn new(id: usize, username: String, password: String) -> Self {
        User {
            id,
            username,
            password,
        }
    }
}