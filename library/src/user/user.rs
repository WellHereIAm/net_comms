/// Struct to hold data about user, most likely will grow in future by a lot.
#[derive(Debug)]
pub struct User {
    id: usize,
    username: String,
    password: String, // This of course needs to be hashed.
}

impl Default for User {

    fn default() -> Self {
        User {
            id: 1,
            username: "default_user".to_string(),
            password: "default".to_string(),
        }
    }
}

impl User {

    /// Creates new User.    
    pub fn new(id: usize, username: String, password: String) -> Self {
        User {
            id,
            username,
            password,
        }
    }

    /// Returns users id.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns users username.
    pub fn username(&self) -> String {
        self.username.clone()
    }

    /// Returns users
    // Not to sure about this one.
    pub fn password(&self) -> String {
        self.password.clone()
    }
}