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
                let cmd: Vec<String> = input(msg).unwrap()
                                        .split_inclusive(" ")
                                        .map(|cmd| {String::from(cmd)})
                                        .collect();
                CommandRaw{vec: cmd}
            },
            None => {
                let cmd: Vec<String> = input(" ").unwrap()
                                        .split_inclusive(" ")
                                        .map(|cmd| {String::from(cmd)})
                                        .collect();
                CommandRaw{vec: cmd}
            },
        };

        dbg!(&cmd);
        cmd
    }



    /// This method consumes the whole CommandRaw struct.    
    pub fn process(mut self, user: &User) -> Result<Command, NetCommsError> {

        match self.vec.get_mut(0) {
            Some(cmd) => {
                let cmd = cmd.replace(" ", "");
                match cmd.as_str() {
                    "register" => {
                        // Later solve situations where check returns an Err value.
                        let user = CommandRaw::check_register(self).unwrap();
                        return Ok(Command::Register(user))
                    },
                    "login" => {
                        // Later solve situations where check returns an Err value.
                        let user = CommandRaw::check_login(self).unwrap();
                        return Ok(Command::Login(user))
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
                        return Ok(msg.unwrap())
                    },
                    _ => {
                        // Maybe throw some error?
                        println!("Unknown command.");
                        return Ok(Command::Unknown)
                    },   
                }
            },
            None => todo!(),
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

        let mut cmd_iter = cmd.vec.iter();
        
        // Used to skip send part, so it´s not added as recipient. Later will use 'if let'
        match cmd_iter.next() {
            Some(v) => {
                dbg!(v);
            },
            None => {
                // Return an error.
            }
        }

        // Get all recipients.
        let mut recipients: Vec<String> = Vec::new();
        let mut is_first = true;
        let mut multiple_recipients = false;
        loop {
            match cmd_iter.next() {
                Some(part) => {

                    dbg!(&part);

                    if part.as_str() == " " {
                        continue;
                    }

                    let recipient = part.replace(" ", "");

                    if is_first {
                        is_first = false;

                        if part.starts_with("(") {
                            multiple_recipients = true;
                        } 
                    }

                    if multiple_recipients {
                        let mut is_last = false;
                        if recipient.ends_with(")") {
                            is_last = true;
                        }

                        if recipient.contains(",") {
                            let recipients_part: Vec<String> = recipient.split(",")
                                                                   .map(|x| String::from(x))
                                                                   .collect();

                            for recipient in recipients_part {
                                let recipient = Self::remove_invalid(recipient);
                                if !recipient.is_empty() {
                                    recipients.push(recipient);
                                }

                            }
                        } else {
                            let recipient = Self::remove_invalid(recipient);
                            if !recipient.is_empty() {
                                recipients.push(recipient);
                            }
                        }
        
                        if is_last {
                            break;
                        }
                    } else {
                        let recipient = Self::remove_invalid(recipient);

                        if !recipient.is_empty() {
                            recipients.push(recipient);
                        }
                        break;
                    }
                },
                None => {
                    // Return an IOError as this command was used wrongly.
                },
            }
        }

        if recipients.is_empty() {
            // Return an error.
        }

        let cmd_content: String = cmd_iter.map(|string| String::from(string)).collect();

        dbg!(&recipients);
        dbg!(&cmd_content);

        if cmd_content.is_empty()  {
            // Return an error.
        }

        let kind: MessageKind;
        let mut file_name: Option<String> = None;
        let mut content = Vec::new();

        // Check if the content of command is Path
        if cmd_content.starts_with("|") {
            kind = MessageKind::File;
            let cmd_content = cmd_content.replace("|", "");
            let path = Path::new(&cmd_content);
            if path.is_file() {
                match path.to_str() {
                    Some(path) => file_name = Some(path.to_string()),
                    None => {
                        // Return an error,
                    }
                }
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
            file_name,
        ))
    }

    /// Removes all invalid characters.
    // Maybe should return 'Result' to signalize if the returned String is empty.
    fn remove_invalid(recipient: String) -> String {
        let invalid_symbols = [" ", ",", "(", ")"];
        let mut recipient = recipient;

        for symbol in invalid_symbols {
            recipient = recipient.replace(symbol, "");
        }

        recipient        
    }
}
