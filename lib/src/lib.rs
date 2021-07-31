pub mod settings;

mod components {
    pub mod packet;
    pub mod message;
    pub mod request;
    pub mod user;
    pub mod command;

    pub use packet::*;
    pub use message::*;
    pub use request::*;
    pub use user::*;
    pub use command::*;
}

pub use crate::components::*;
pub use crate::settings::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }   
    
}
