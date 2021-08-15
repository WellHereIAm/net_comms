use std::fmt::{Debug, Display};
use std::error::Error;

use crate::error_testing::NetCommsErrorType;

pub enum BlankError {}

impl Debug for BlankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for BlankError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for BlankError {}

impl NetCommsErrorType for BlankError {}

impl BlankError {
    
    pub fn output(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}