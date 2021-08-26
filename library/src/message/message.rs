use serde::{Serialize, Deserialize};

use std::fmt::{Debug, Display};
use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};


use crate::error::{NetCommsError};
use crate::packet::{Packet};
use crate::ron::{FromRon, IntoRon};

use super::{ContentType, MessageKindType, MetaDataType};

/// Message<'a, K, M, C>
///     where
///     K: MessageKindType<'a>,
///     M: MetaDataType<'a>,
///     C: ContentType<'a, K, M, C>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<K, M, C> {
    kind: K,
    metadata: M,
    content: C, 
    end_data: Packet,
}

impl<K, M, C> Default for Message<K, M, C> 
where
    K: Default,
    M: Default,
    C: Default {
    
    fn default() -> Self {
        Message {
            kind: K::default(),
            metadata: M::default(),
            content: C::default(),
            end_data: Packet::default(),
        }
    }
}

impl<K, M, C> Display for Message<K, M, C>
where
    K: Serialize,
    M: Serialize,
    C: Serialize {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let formatted = self.into_ron_pretty(None).expect("Failed to parse a Message to RON.");
        write!(f, "{}", &formatted)
    }
}

impl<'a, K, M, C> IntoRon for Message<K, M, C> 
where
    K: Serialize,
    M: Serialize,
    C: Serialize {}

impl<'a, K, M, C> FromRon<'a> for Message<K, M, C> 
where
    K: Deserialize<'a>,
    M: Deserialize<'a>,
    C: Deserialize<'a> {}


    
impl<'a, K, M, C> Message<K, M, C> 
where
    K: Default + Clone + MessageKindType<'a>,
    M: Default + Clone + MetaDataType<'a>,
    C: Default + Clone + ContentType<'a, K, M, C> {

    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn send(self, stream: &mut TcpStream) -> Result<(), NetCommsError> {
        
        let metadata = self.metadata.send(stream)?;

        self.content.send(stream, metadata)?;

        self.end_data.send(stream)?;

        Ok(())
    }

    pub fn receive(stream: &mut TcpStream, location: Option<PathBuf>) -> Result<Self, NetCommsError> {
        
        let mut message = Self::default();

        let metadata = M::receive(stream, location.clone())?;
        message.set_metadata(metadata);

        let (content, end_data) = C::receive(stream, &mut message, location)?;
        message.set_content(content);
        message.set_end_data(end_data);

        Ok(message)
    }

/// Implementation of setters and getters for [Message].


    pub fn kind(&self) -> K {
        self.kind.clone()
    }

    pub fn kind_move(self) -> K {
        self.kind
    }

    pub fn metadata(&self) -> M {
        self.metadata.clone()
    }

    pub fn metadata_mut(&'a mut self) -> &'a M {
        &self.metadata
    }

    pub fn metadata_move(self) -> M {
        self.metadata
    }

    pub fn content_ref<'b>(&'b self) -> &'b C {
        &self.content
    }

    pub fn content_mut<'b>(&'b mut self) -> &'b mut C {
        &mut self.content
    }

    pub fn content_move(self) -> C {
        self.content
    }

    pub fn set_metadata(&mut self, metadata: M) {
        self.metadata = metadata;
    }

    pub fn set_content(&mut self, content: C) {
        self.content = content;
    }

    pub fn set_end_data(&mut self, end_data: Packet) {
        self.end_data = end_data;
    }

    pub fn save(&self, location: &Path) {

        let message_ron = self.into_ron().unwrap();
        fs::create_dir_all(location.parent().unwrap()).unwrap();

        let mut file = fs::OpenOptions::new().create(true).write(true).open(location).unwrap();
        file.write_fmt(format_args!("{}", message_ron)).unwrap();
    }
}