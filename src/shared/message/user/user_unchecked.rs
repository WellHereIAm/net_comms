use serde::{Serialize, Deserialize};

/// Holds data about user that do not need to be valid so are used inside
/// [requests](crate::request::Request), so server can check it.
///
/// # Fields
///
/// * `username` 
/// * `password`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnchecked {
    pub username: String,
    pub password: String,
}