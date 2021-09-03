use serde::{Serialize, Deserialize};

use std::fmt::Display;
use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::fs::{self, OpenOptions};

use library::bytes::{Bytes, FromBytes, IntoBytes};
use library::error::NetCommsErrorKind;
use library::ron::FromRon;
use library::{message::{ContentType}, packet::Packet, prelude::{IntoRon, Message, NetCommsError}};

use super::{message_kind::MessageKind, metadata::MetaData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content (String);

impl Default for Content {

    fn default() -> Self {
        Self::new()
    }
}

impl FromRon<'_> for Content {}
impl IntoRon for Content {}

impl Display for Content {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Content({})", self.string_ref())
    }
}

impl IntoBytes for Content {

    fn into_bytes(self) -> Bytes {
        Bytes::from_vec(self.0.into_bytes())
    }
}

impl FromBytes for Content {

    fn from_bytes(bytes: Bytes) -> Result<Self, NetCommsError>
    where
        Self: Sized {

        Ok(Content::with_data(bytes.to_string()))
    }

    fn from_buff(buff: &[u8]) -> Result<Self, NetCommsError>
    where
        Self: Sized {
        todo!()
    }
}


impl ContentType<'_, MessageKind, MetaData, Content> for Content {
    
    fn send(self, stream: &mut TcpStream, metadata: MetaData) -> Result<(), NetCommsError> {

        match metadata.file_name() {
            Some(file_name) => {
                let path = Path::new(&file_name);
                Message::<MessageKind, MetaData, Content>::send_file(stream, path)?;
            }
            None => {
                let bytes = self.0.as_bytes().to_vec().into_bytes();
                Message::<MessageKind, MetaData, Content>::send_content(stream, bytes)?
            },
        }

        Ok(())
    }

    fn receive(stream: &mut TcpStream,
               message: &mut Message<MessageKind, MetaData, Content>,
               path: Option<PathBuf>) -> Result<(Self, Packet), NetCommsError> {

        let path = path.unwrap();
        let mut path = message.metadata().get_message_location(&path);

        if let Err(e) = fs::create_dir_all(&path) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::CreatingDirFailed,
                Some(format!("Could not create a directory on {}. \n({})",
                                     &path.parent().unwrap().to_str().unwrap(), e))));
        }

        let (content, end_data) = match message.metadata().message_kind() {
            MessageKind::File => {
                let (_, end_data) = Message::<MessageKind, MetaData, Content>::receive_file(
                    stream,
                    &path,
                    message
                                .metadata_ref()
                                .file_name()
                                .unwrap()
                )?;
                (Content::new(), end_data)
            }
            _ => {
                let (bytes, end_data) = Message::<MessageKind, MetaData, Content>::receive_content(stream)?;
                let content = Content::with_data(bytes.to_string());
                (content, end_data)
            }
        };

        message.set_content(content.clone());
        message.set_end_data(end_data.clone());

        let message_ron = message.into_ron()?;        
        path.push("message.ron");
        let mut file = fs::OpenOptions::new().create(true).write(true).open(path).unwrap();
        file.write_fmt(format_args!("{}", message_ron)).unwrap();

        Ok((content, end_data))
    }
}


impl Content {

    pub fn new() -> Self {
        Content(String::new())
    }

    pub fn with_data(data: String) -> Self {
        Content(data)
    }
    
    pub fn append_string(&mut self, string: String) {
        for char in string.chars() {
            self.0.push(char);
        }
    }
    
    pub fn into_string(self) -> String {
        self.0
    }

    pub fn string_ref<'a>(&'a self) -> &'a String {
        &self.0
    }
}
