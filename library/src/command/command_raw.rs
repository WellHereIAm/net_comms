use std::path::Path;

use utils::input;

use crate::buffer::ToBuffer;
use crate::command::Command;
use crate::prelude::NetCommsError;
use crate::user::{User, UserUnchecked};
use crate::message::MessageKind;


/// CommandRaw holds a vector of strings, parts of inputted command.
#[derive(Debug)]
pub struct CommandRaw{
    pub vec: Vec<String>,
}

impl CommandRaw {

    /// Gets an input from the user and splits it on every whitespace.
    pub fn get<T>(msg: Option<T>) -> Self
    where 
        T: std::fmt::Display {

    let cmd = match msg {
        Some(msg) => {
            let cmd = input(msg).unwrap()
                                    .split_whitespace()
                                    .map(|cmd| {String::from(cmd)})
                                    .collect::<Vec<String>>();
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

    // pub fn vec(&self) -> Vec<u8> {
    //     self.vec.clone()
    // }

    // pub fn vec_owned(self) -> Vec<u8> {
    //     self.vec
    // }

    // This function consumes the whole CommandRaw struct.    
    pub fn process(self, user: &User) -> Result<Command, NetCommsError> {
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
                let msg = CommandRaw::check_send(self, &user);
                Ok(msg.unwrap())
            },
            _ => {
                // Maybe throw some error?
                println!("Unknown command.");
                Ok(Command::Unknown)
            },                
        }
    }

    fn check_register(cmd: CommandRaw) -> Result<UserUnchecked, NetCommsError> {
        // Later will perform logic to check if inputted command is a valid register command.
        Ok(UserUnchecked {
            username: cmd.vec[1].clone(),
            password: cmd.vec[2].clone(),
        })
    }

    fn check_login(cmd: CommandRaw) -> Result<UserUnchecked, NetCommsError> {
        // Later will perform logic to check if inputted command is a valid login command.
        Ok(UserUnchecked {
            username: cmd.vec[1].clone(),
            password: cmd.vec[2].clone(),
        })
    }

    fn check_yes(_cmd: CommandRaw) -> Result<Command, NetCommsError> {
        // Later will perform logic to check if inputted command is a valid yes command.
        Ok(Command::Yes)
    }

    fn check_no(_cmd: CommandRaw) -> Result<Command, NetCommsError> {
        // Later will perform logic to check if inputted command is a valid no command.
        Ok(Command::No)
    }

    fn check_send(cmd: CommandRaw, user: &User) -> Result<Command, NetCommsError> {

        let mut recipients: Vec<String> = Vec::new();
        let mut recipients_length = 0;
        let cmd_len = cmd.vec.len();

        match cmd.vec.get(1) {
            Some(string) => {

                if string.starts_with("(") {

                    let mut is_last = false;

                    for recipient in cmd.vec[1..cmd_len].to_vec() {
                        
                        if recipient.ends_with(")") {
                            is_last = true;
                        }

                        let recipient = recipient.replace("(", "");
                        let recipient = recipient.replace(")", "");

                        let recipients_part: Vec<String> = recipient.split(",").map(|rec| rec.to_string()).collect();

                        for recipient in recipients_part {
                            if recipient.len() > 0 {
                                recipients.push(recipient);
                                recipients_length += 1;
                            }
                        }

                        if is_last {
                            break;
                        }
                    }
                } else {
                    recipients_length += 1;
                    recipients.push(cmd.vec[1].clone());
                }
            }
            None => {
                // Invoke error.
                todo!();
            },
        }

        let does_exist: bool;
        match cmd.vec.get(recipients_length + 1) {
            Some(_) => {
                does_exist = true;
            },
            None => {
                does_exist = false
            }, // Invoke error
        }

        let mut cmd_content = String::new();
        
        if does_exist {
            for mut part in cmd.vec[recipients_length + 1..cmd_len].to_vec() {
                part.push(' ');
                cmd_content.push_str(&part);          
            }
        }

        let kind: MessageKind;
        let mut file_name: Option<String> = None;
        let mut content = Vec::new();

        // Check if the content of command is Path
        if cmd_content.starts_with("|") {
            kind = MessageKind::File;
            cmd_content = cmd_content.replace("|", "");
            let path = Path::new(&cmd_content);
            if path.is_file() {
                // Rework this for proper Error handling.
                file_name = Some(path.to_str().unwrap().to_string());    
            }
        } else {
            kind = MessageKind::Text;
            file_name = None;
            content = cmd_content.to_buff()?;
        }

        let author_id = user.id();

        Ok(Command::Send(
            kind,
            author_id,
            recipients,
            content,
            file_name, // Should rename this
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
