use std::path::Path;

use utils::input;

use library::buffer::IntoBuffer;
use library::error::{NetCommsError, NetCommsErrorKind};
use library::message::MessageKind;
use library::user::{User, UserUnchecked};

use super::Command;



/// Is used to get user input through its [CommandRaw::get]
/// # Fields
/// 
/// `vec` -- User input split by whitespace, with the whitespaces included.
#[derive(Debug)]
pub struct CommandRaw {
    pub vec: Vec<String>, // I would like to change this to iterator in the future.
}

impl CommandRaw {

    /// Gets the user input from command line and return [CommandRaw].
    ///
    /// # Arguments
    /// 
    /// `msg` -- Option of anything that implements [std::fmt::Display], if some it is printed to the console.
    ///
    /// # Examples 
    ///
    /// ```
    /// let command_raw = CommandRaw::get(Some("Write your command: \n".to_string()));
    /// ```
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
                // Here is an empty String instead of msg from argument.
                let cmd: Vec<String> = input("").unwrap()
                                        .split_inclusive(" ")
                                        .map(|cmd| {String::from(cmd)})
                                        .collect();
                CommandRaw{vec: cmd}
            },
        };
        cmd
    }



    /// This method takes an ownership of [self] and returns [Command] if successful.
    /// 
    /// # Arguments
    ///
    /// `user` -- A reference to [User] from whom this command should come from. 
    ///
    /// # Examples 
    ///
    /// ```
    /// // Users are not usually and should not be created like that,
    /// // here it is used only for purpose of this example.
    /// let user = User::new(1, "some_username".to_string(), "some_password".to_string());
    /// let command_raw = CommandRaw::get(Some("Write your command: \n".to_string()));
    /// ```
    /// Here is created a [Command], that can be send using [Message](crate::message::Message).
    /// ```
    /// # let user = User::new(1, "some_username".to_string(), "some_password".to_string());
    /// # let command_raw = CommandRaw::get(Some("Write your command: \n".to_string()));
    /// let command = command_raw.process().unwrap(); 
    /// ```
    ///
    /// # Errors
    /// 
    /// * Usual cause of error inside this method is [InvalidCommand](NetCommsErrorKind::InvalidCommand) or [UnknownCommand](NetCommsErrorKind::UnknownCommand)
    /// which are caused by user invalid user input, those are recoverable errors.
    /// * This can also return other [NetCommsError].
    pub fn process(mut self, user: &User) -> Result<Command, NetCommsError> {

        // Match for known commands.
        match self.vec.get_mut(0) {
            Some(cmd) => {
                let cmd = cmd.replace(" ", "");
                match cmd.as_str() {
                    "register" => {
                        let user_unchecked = CommandRaw::check_register(self)?;
                        return Ok(Command::Register(user_unchecked, user.clone()))
                    },
                    "login" => {
                        // Later solve situations where check returns an Err value.
                        let user_unchecked = CommandRaw::check_login(self).unwrap();
                        return Ok(Command::Login(user_unchecked, user.clone()))
                    },
                    "y" => {
                        // Finish check function
                        match CommandRaw::check_yes(self) {
                            Ok(_) => return Ok(Command::Yes(user.clone())),
                            Err(_) => todo!(),
                            
                        };
                    },
                    "n" => {
                        // Finish check function
                        match CommandRaw::check_no(self) {
                            Ok(_) => return Ok(Command::No(user.clone())),
                            Err(_) => todo!(),
                            
                        };
                    },
                    "send" => {
                        let send_cmd = CommandRaw::check_send(self, &user)?;
                        return Ok(send_cmd)
                    },
                    _ => {
                        return Err(NetCommsError::new(
                            NetCommsErrorKind::UnknownCommand,
                            None));
                    },   
                }
            },
            None => return Err(NetCommsError::new(
                NetCommsErrorKind::UnknownCommand,
                None)),
        }
    }

    /// Checks if given command is valid register command.
    fn check_register(cmd: CommandRaw) -> Result<UserUnchecked, NetCommsError> {

        let mut cmd_vec: Vec<String> = cmd.vec
                                      .iter()
                                      // Removes invalid characters.
                                      .map(|x| Self::remove_invalid(x.to_owned())) 
                                      // Removes first, "register", element.
                                      .filter(|x| x.as_str() != "register") 
                                      .collect();

        // Safety check if the command has correct length.
        if cmd_vec.len() < 3 {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidCommand, 
                Some("Command register does not have all its parts.".to_string())));
        }

        // Because Vec::remove moves all contents to the left, to get password, it is needed to use index 0 again.
        let username = cmd_vec.remove(0);
        let password: String;
        if cmd_vec[0] == cmd_vec[1] {
            password = cmd_vec.remove(0);
        } else {
            return Err(NetCommsError::new(
                NetCommsErrorKind::UnknownCommand,
                Some("Passwords do not match.".to_string())
            ));
        }

        Ok(UserUnchecked {
            username,
            password,
        })
    }

    /// Checks if given command is valid register command.
    fn check_login(cmd: CommandRaw) -> Result<UserUnchecked, NetCommsError> {

        let mut cmd_vec: Vec<String> = cmd.vec
                                      .iter()
                                      // Removes invalid characters.
                                      .map(|x| Self::remove_invalid(x.to_owned())) 
                                      // Removes first, "login", element.
                                      .filter(|x| x.as_str() != "login") 
                                      .collect();

        // Safety check if the command has correct length.
        if cmd_vec.len() < 2 {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidCommand, 
                Some("Command login does not have all its parts.".to_string())));
        }

        // Because Vec::remove moves all contents to the left, to get password, it is needed to use index 0 again.
        let username = cmd_vec.remove(0);
        let password = cmd_vec.remove(0);

        Ok(UserUnchecked {
            username,
            password,
        })
    }

    fn check_yes(_cmd: CommandRaw) -> Result<Command, NetCommsError> {
        todo!()
        // Later will perform logic to check if inputted command is a valid yes command.
        // Ok(Command::Yes)
    }

    fn check_no(_cmd: CommandRaw) -> Result<Command, NetCommsError> {
        todo!()
        // Later will perform logic to check if inputted command is a valid no command.
        // Ok(Command::No)
    }

    /// Checks if given command is valid send command.
    fn check_send(cmd: CommandRaw, user: &User) -> Result<Command, NetCommsError> {

        let mut cmd_iter = cmd.vec.iter();
        
        // Used to skip send part, so it´s not added as recipient.
        if let None = cmd_iter.next() {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidCommand, 
                None));
        }

        // Get all recipients.
        let mut recipients: Vec<String> = Vec::new();
        let mut is_first = true;
        let mut multiple_recipients = false;
        loop {
            match cmd_iter.next() {
                Some(part) => {
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
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidCommand, 
                        Some("command \"send\" needs to be followed by recipient/s and content.".to_string())));
                },
            }
        }

        if recipients.is_empty() {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidCommand, 
                Some("No recipients in \"send\" command.".to_string())));
        }

        // Change content to owned String.
        let cmd_content: String = cmd_iter.map(|string| String::from(string)).collect();

        if cmd_content.is_empty()  {
            return Err(NetCommsError::new(
                NetCommsErrorKind::InvalidCommand, 
                Some("No content in \"send\" command.".to_string())));
        }

        let kind: MessageKind;
        let mut file_name: Option<String> = None;
        let mut content = Vec::new();

        // Check if the content of command is a Path
        if cmd_content.starts_with("|") {
            kind = MessageKind::File;
            let cmd_content = cmd_content.replace("|", "");
            let path = Path::new(&cmd_content);
            if path.is_file() {
                match path.to_str() {
                    Some(path) => file_name = Some(path.to_string()),
                    None => {
                        return Err(NetCommsError::new(
                            NetCommsErrorKind::InvalidCommand, 
                            Some("Given file does not exist.".to_string())));
                    }
                }
            }
        } else {
            kind = MessageKind::Text;
            file_name = None;
            content = cmd_content.into_buff()?;
        }

        let author = user.clone();

        Ok(Command::Send(
            kind,
            author,
            recipients,
            content,
            file_name,
        ))
    }

    /// Removes all invalid characters.
    fn remove_invalid(string: String) -> String {
        let invalid_symbols = [" ", ",", "(", ")"];
        let mut recipient = string;

        for symbol in invalid_symbols {
            recipient = recipient.replace(symbol, "");
        }

        recipient        
    }
}
