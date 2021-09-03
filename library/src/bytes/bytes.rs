use serde::{Serialize, Deserialize};

use std::{ops::{Deref, Index, IndexMut, Range}, slice::SliceIndex, vec::IntoIter};

use crate::ron::{IntoRon, FromRon};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bytes(Vec<u8>);

impl Default for Bytes {

    fn default() -> Self {
        Bytes(Vec::new())
    }
}

impl<Idx> Index<Idx> for Bytes
where
    Idx: SliceIndex<[u8]> {
        
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.as_slice()[index]
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

    pub fn from_arr<const L: usize>(arr: [u8; L]) -> Self {

        let mut bytes = Bytes::new();
        for byte in arr.iter() {
            bytes.push(*byte);
        }
        bytes
    }

    pub fn push(&mut self, value: u8) {
        self.0.push(value)
    }

    pub fn as_slice<'a>(&'a self) -> &'a[u8] {
        &self.0
    }

    pub fn vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn vec_ref<'a>(&'a self) -> &'a Vec<u8> {
        &self.0
    }

    pub fn vec_mut<'a>(&'a mut self) -> &'a mut Vec<u8> {
        &mut self.0
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_iter(self) -> IntoIter<u8>  {
        self.0.into_iter()
    }

    pub fn append(&mut self, other: &mut Bytes) {
        self.0.append(other.vec_mut())
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        let value = self.as_slice().get(index);
        let value = match value {
            Some(value) => Some(*value),
            None => None,
        };

        value
    }
}