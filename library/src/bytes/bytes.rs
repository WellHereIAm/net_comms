use serde::{Serialize, Deserialize};

use std::iter::FromIterator;
use std::ops::Index;
use std::slice::SliceIndex;

use crate::ron::{ToRon, FromRon};

/// Wrapper around [Vec] of [u8] used to store data in form of bytes in this library.
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

impl IntoIterator for Bytes {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<u8> for Bytes {

    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        
        let mut bytes = Bytes::new();

        for byte in iter {
            bytes.push(byte)
        }

        bytes
    }
}

impl FromRon<'_> for Bytes {}
impl ToRon for Bytes {}

impl Bytes {
    
    /// Creates a new empty [Bytes].
    ///
    /// Usually declared as mutable and filled later.
    pub fn new() -> Self {
        Bytes(Vec::new())
    }

    /// Creates [Bytes] from [Vec] of [u8].
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Bytes(vec)
    }

    /// Creates [Bytes] from an [array] of [u8].
    pub fn from_arr<const L: usize>(arr: [u8; L]) -> Self {

        let bytes: Bytes = arr.iter().map(|byte| *byte).collect();
        bytes
    }

    /// Pushes a new [u8] into [Bytes].
    ///
    /// Wrapper around [Vec::push].
    pub fn push(&mut self, value: u8) {
        self.0.push(value)
    }

    /// Returns an immutable [slice] of Bytes.
    pub fn as_slice<'a>(&'a self) -> &'a[u8] {
        &self.0
    }

    /// Returns an inner [Vec].
    ///
    /// [Vec] is cloned.
    pub fn vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    /// Returns an reference to inner [Vec].
    pub fn vec_ref<'a>(&'a self) -> &'a Vec<u8> {
        &self.0
    }

    /// Returns an mutable reference to inner [Vec].
    pub fn vec_mut<'a>(&'a mut self) -> &'a mut Vec<u8> {
        &mut self.0
    }

    /// Takes an ownership of `self` and returns an inner [Vec].
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    /// Returns number of bytes inside `Self`.
    ///
    /// Wrapper around [Vec::len].
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Moves all elements of `other` into `Self`, leaving `other` empty.
    ///
    /// Wrapper around [Vec::append].
    pub fn append(&mut self, other: &mut Bytes) {
        self.0.append(other.vec_mut())
    }

    /// Returns an valid UTF-8 [String].
    ///
    /// Since it is a wrapper around [String::from_utf8_lossy] it just replaces all
    /// invalid bytes with `U+FFFD REPLACEMENT CHARACTER`.
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }

    /// Returns [Some] with [u8] inside if there is a value on given `index`,
    /// otherwise returns [None].
    pub fn get(&self, index: usize) -> Option<u8> {
        let value = self.as_slice().get(index);
        let value = match value {
            Some(value) => Some(*value),
            None => None,
        };

        value
    }
}