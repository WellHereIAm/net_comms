use serde::{Serialize, Deserialize};

use crate::ron::{IntoRon, FromRon};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bytes(Vec<u8>);

impl Default for Bytes {

    fn default() -> Self {
        Bytes(Vec::new())
    }
}

impl FromRon<'_> for Bytes {}
impl IntoRon for Bytes {}

impl Bytes {
    
    pub fn new() -> Self {
        Bytes(Vec::new())
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        Bytes(vec)
    }

    pub fn push(&mut self, value: u8) {
        self.0.push(value)
    }

    pub fn get<'a>(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}