use std::fmt::{Debug, Display};
use std::error::Error;

use backtrace::Backtrace;

pub trait NetCommsErrorType: Error {
    fn backtrace() -> Backtrace {
        Backtrace::new()
    }
}

pub struct NetCommsError<E>
where E: NetCommsErrorType 
{
    pub kind: E,
    pub message: Option<String>,
    pub backtrace: Backtrace,
}

impl<E> Debug for NetCommsError<E> 
where E: NetCommsErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<E> Display for NetCommsError<E> 
where E: NetCommsErrorType {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<E> Error for NetCommsError<E>
where E: NetCommsErrorType {}

impl<E> NetCommsError<E> 
where E: NetCommsErrorType
{
    pub fn new(kind: E, message: Option<String>) -> NetCommsError<E> {

        let backtrace =  <E as NetCommsErrorType>::backtrace(); // Well...

        NetCommsError {
            kind,
            message,
            backtrace,
        }        
    }

    fn output(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

