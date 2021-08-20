use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnchecked{
    pub username: String,
    pub password: String,
}