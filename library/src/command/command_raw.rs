use utils::input;

use crate::command::{Command, CommandError};
use crate::user::UserUnchecked;
use crate::message::MessageKind;


/// CommandRaw holds a vector of strings, parts of inputted command.
pub struct CommandRaw{
    vec: Vec<String>,
}

impl CommandRaw {

    pub fn get<T>(msg: Option<T>) -> Self
    where 
        T: std::fmt::Display {


    // Maybe later change input to macro, so it won´t be so ugly.
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

        let author_id = 1; // LATER GET DYNAMIC IDS
        let file_name = None; // GET THIS DYNAMICALLY.
        Ok(Command::Send(
            kind,
            author_id,
            recipients,
            content,
            file_name,
        ))
    }
}

#[derive(Debug)]
pub struct CommandRawError;

impl std::fmt::Display for CommandRawError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandRawError - TODO")
    }
}

impl std::error::Error for CommandRawError {}
