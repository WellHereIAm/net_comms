use std::hash::Hash;

use library::prelude::FromRon;
use library::prelude::ToRon;
use pbkdf2::Pbkdf2;
use pbkdf2::password_hash::PasswordHash;
use pbkdf2::password_hash::PasswordHasher;
use pbkdf2::password_hash::PasswordVerifier;
use pbkdf2::password_hash::SaltString;
use rand_core::OsRng;
use serde::{Serialize, Deserialize};

use rand::{distributions::Alphanumeric, Rng};

use ron::ser;
use ron::de;


use library::error::{NetCommsError, NetCommsErrorKind};
use crate::config::SERVER_ID;
use crate::config::SERVER_USERNAME;
use crate::config::{UNKNOWN_USER_ID, UNKNOWN_USERNAME};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthToken (String);

impl AuthToken {
    
    fn new() -> Self {

        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        println!("{}", s);

        AuthToken (s)
    }


}

# [test]
fn auth_token() {

    let token = AuthToken::new();
    let t = token.clone();
    println!("{:?}", token);
    assert_eq!(t, token);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password (String);

impl Password {
    
    pub fn new(raw_password: String) -> Self {
        
        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($pbkdf2-sha256$...)
        let password_hash = Pbkdf2.hash_password(raw_password.as_bytes(), &salt).unwrap().to_string();

        Password (password_hash)
    }

    pub fn from_hash(hash: String) -> Self {
        Password (hash)
    }

    pub fn verify(&self, other: String) -> bool {
        
        let password_hash = PasswordHash::new(&self.0).unwrap();
        
        if Pbkdf2.verify_password(other.as_bytes(), &password_hash).is_ok() {
            true
        } else {
            false
        } 
    }
}

# [test]
fn password() {
    
    let password_raw = String::from("password");
    let password = Password::new(password_raw);
    println!("{:?}", &password);

    assert!(password.verify("not".to_string()));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLite {
    id: u32,
    username: String
}

impl UserLite {

    pub fn new(id: u32, username: String) -> Self {
        UserLite {
            id,
            username,
        }
    }
    
    pub fn from_user(user: &User) -> Self {
        UserLite { 
            id: user.id(),
            username: user.username()
        }
    }

    pub fn default_user() -> Self {
        UserLite {
            id: UNKNOWN_USER_ID,
            username: UNKNOWN_USERNAME.to_string(),
        }
    }

    pub fn default_server() -> Self {
        UserLite {
            id: SERVER_ID,
            username: SERVER_USERNAME.to_string(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }
}

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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: u32,
    username: String,
    password: Password, 
    auth_token: Option<AuthToken>,
}

impl FromRon<'_> for User {}
impl ToRon for User {}


impl User {
  
    pub fn new(id: u32, username: String, password: Password) -> Self {
        User {
            id,
            username,
            password,
            auth_token: None
        }
    }

    pub fn from_user_unchecked(id: u32, user_unchecked: UserUnchecked) -> Self {
        User {
            id,
            username: user_unchecked.username,
            password: Password::new(user_unchecked.password),
            auth_token: None
        }
    }

    pub fn verify(&self, other: String) -> bool {
        self.password.verify(other)
    }

    /// Returns `users_id`.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns `username`.
    pub fn username(&self) -> String {
        self.username.clone()
    }
}