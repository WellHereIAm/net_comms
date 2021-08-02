use std::str::Bytes;

use utils::input;

use crate::{Message, components::user::UserUnchecked, settings, MessageKind};

pub enum Command {
    Register(UserUnchecked),
    Login(UserUnchecked),
    Yes,
    No,
    Send(MessageKind ,Vec<String>, Vec<u8>),    // Send commands have also info about message kind, recipients and content
    Unknown
}

#[derive(Debug)]
pub struct CommandError;

impl std::fmt::Display for CommandError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandError - TODO")
    }
}

impl std::error::Error for CommandError {}

pub struct CommandRaw{
    vec: Vec<String>,
}

#[derive(Debug)]
pub struct CommandRawError;

impl std::fmt::Display for CommandRawError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandRawError - TODO")
    }
}

impl std::error::Error for CommandRawError {}

impl CommandRaw {

    pub fn get<T>(msg: Option<T>) -> Self
    where 
        T: std::fmt::Display {


    // Maybe later change input to macro so it won´t be so ugly.
    let cmd = match msg {
        Some(msg) => {
            let cmd = input(msg).unwrap()
                                    .split_whitespace()
                                    .map(|cmd| {String::from(cmd)}).collect::<Vec<String>>();
            CommandRaw{vec: cmd}
        },
        None => {
            let cmd = input("").unwrap()
                                    .split_whitespace()
                                    .map(|cmd| {String::from(cmd)}).collect::<Vec<String>>();
            CommandRaw{vec: cmd}
        },
    };
    cmd
    }

    // This function consumes the whole CommandRaw struct.    
    pub fn process_cmd(self) -> Result<Command, CommandError> {
        // ERROR HANDLING!
        match self.vec[0].as_str() {
            "register" => {
                // Later solve situations where check returns an Err value.
                let user = CommandRaw::check_register(self).unwrap();
                Ok(Command::Register(user))
            },
            "login" => {
                // Later solve situations where check returns an Err value.
                let user = CommandRaw::check_login(self).unwrap();
                Ok(Command::Login(user))
            },
            "y" => {
                // Finish check function
                match CommandRaw::check_yes(self) {
                    Ok(_) => return Ok(Command::Yes),
                    Err(_) => todo!(),
                    
                };
            },
            "n" => {
                // Finish check function
                match CommandRaw::check_no(self) {
                    Ok(_) => return Ok(Command::No),
                    Err(_) => todo!(),
                    
                };
            },
            "send" => {
                let msg = CommandRaw::check_send(self);
                Ok(msg.unwrap())
            },
            _ => {
                // Maybe throw some error?
                println!("Unknown command.");
                Ok(Command::Unknown)
            },                
        }
    }

    fn check_register(cmd: CommandRaw) -> Result<UserUnchecked, CommandRawError> {
        // Later will perform logic to check if inputted command is a valid register command.
        Ok(UserUnchecked {
            username: cmd.vec[1].clone(),
            password: cmd.vec[2].clone(),
        })
    }

    fn check_login(cmd: CommandRaw) -> Result<UserUnchecked, CommandRawError> {
        // Later will perform logic to check if inputted command is a valid login command.
        Ok(UserUnchecked {
            username: cmd.vec[1].clone(),
            password: cmd.vec[2].clone(),
        })
    }

    fn check_yes(cmd: CommandRaw) -> Result<Command, CommandRawError> {
        // Later will perform logic to check if inputted command is a valid yes command.
        Ok(Command::Yes)
    }

    fn check_no(cmd: CommandRaw) -> Result<Command, CommandRawError> {
        // Later will perform logic to check if inputted command is a valid no command.
        Ok(Command::No)
    }

    fn check_send(cmd: CommandRaw) -> Result<Command, CommandRawError> {
        // Later will perform logic to check if inputted command is a valid send command.
        let kind = MessageKind::Text; // Later deduct kind based of the content
        let recipients = vec![cmd.vec[1].clone()]; // Later process it to remove parentheses to allow multiple receivers.
        let content = Vec::from(cmd.vec[2].as_bytes());
        Ok(Command::Send(
            kind,
            recipients,
            content,
        ))
    }
}
