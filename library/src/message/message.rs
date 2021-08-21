use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;

use serde::{Serialize, Deserialize};
use itertools::Itertools;

use crate::buffer::{ToBuffer, FromBuffer};
use crate::error::{NetCommsError, NetCommsErrorKind};
use crate::command::Command;
use crate::message::MessageKind;
use crate::packet::{MetaData, PacketKind, Packet};
use crate::config::{MAX_PACKET_CONTENT_SIZE, MAX_PACKET_SIZE, SERVER_ID, SERVER_USERNAME};
use crate::prelude::{Request, User, UserUnchecked};


/// Container to hold all data about the message.
///
/// This is a key struct as it is used to send every kind of message, even though every data sent
/// are sent like [packets](Packet) which are further transformed lower to buffer, both client and
/// server are using [messages](Message) to store information about the data sent and received as
/// they offer complete information which [packets](Packet) by themselves do not.
///
/// # Fields
/// 
/// * `kind` -- [MessageKind] of this [Message]. This is part of metadata as well, but here it eases the access.
///
/// * `metadata` -- [MetaData] if this [Message], holds all information about this [Message].
///
/// * `content` -- [Vector](Vec) of [Packets](Packet) that together contain the whole content of this [Message].
/// Also in case it is the file what was sent, `content` holds information about the file beyond what already
/// is in `metadata`. 
// --- Last bit is not true right now, should be done later. ---
///
/// * `end_data` -- one [Packet] with [PacketKind::End] to sign for end of this [Message]. If more than
/// one end_data [Packet] is sent, only first will be registered, others will be ignored by this implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    kind: MessageKind,
    metadata: MetaData,
    content: Vec<Packet>, // This should hold some info about file, if that is what was sent, like size etc.
    end_data: Packet,   // This should never grow to more than one packet.
}

impl Message {
    
    /// Creates a new empty Message.
    /// 
    /// Created [Message] should always be declared as mutable so other methods can be used to fill it.
    /// Those methods are:
    /// * [Message::set_metadata]
    /// * [Message::push_content]
    /// * [Message::set_end_data]
    ///
    /// # Examples
    ///
    /// ```
    /// let mut message = Message::new().unwrap();
    ///
    /// let content = "Hello there.".to_string().to_buff().unwrap(); 
    /// // Users are not usually and should not be created like that,
    /// // here it is used only for purpose of this example.
    /// let author = User::new(1, "some_username".to_string(), "some_password".to_string());
    /// let recipients = vec!["Fred".to_string(), "Emilia".to_string()];
    /// let metadata = MetaData::new(&content, MessageKind::Text, author, SERVER_ID, recipients, None);
    /// let end_data = Packet::new(PacketKind::End);
    /// 
    /// message.set_metadata(metadata);
    /// message.push_content(Packet::new(PacketKind::new_content(content)));
    /// message.set_end_data(end_data);
    /// ```
    /// # Errors
    /// 
    /// This method should not return an error.
    pub fn new() -> Result<Self, NetCommsError> {
        Ok(Message {
            kind: MessageKind::Unknown,
            metadata: MetaData::new_empty()?,
            content: Vec::new(),
            end_data: Packet::new_empty(),
        })
    }

    /// Creates a new [Message] from [Command].
    ///
    /// # Arguments
    /// 
    /// `command` -- [Command] that should have all information necessary to create a [Message].
    /// # Examples
    /// 
    /// ```
    /// let username = "Francis".to_string();
    /// let password = "superstrongpassword".to_string();
    /// let user_unchecked = UserUnchecked {username, password};
    /// let command = Command::Register(user_unchecked, User::default());
    /// 
    /// let message = Message::from_command(command).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// * This method will return an [NetCommsError] with kind [NetCommsErrorKind::UnknownCommand]
    /// if the given [Command] is not yet supported. Supported commands are [Command::Send], [Command::Register],
    /// [Command::Login].
    /// * Can also return other [NetCommsError] when serializing requests or other structs, or when using [to_buff](crate::buffer::ToBuffer::to_buff)
    /// on some types.
    pub fn from_command(command: Command) -> Result<Self, NetCommsError> {

        // As commands can be created only by client or in client like fashion, recipient_id will always be SERVER_ID.
        match command {
            Command::Send(message_kind, author, recipients, content, file_name) => {
                return Self::from_send(message_kind, author, recipients, content, file_name);    
            }
            Command::Register(user_unchecked, author) => {
                return Self::from_register(user_unchecked, author);
            }
            Command::Login(user_unchecked, author) => {
                return Self::from_login(user_unchecked, author);
            }
            _ => {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::UnknownCommand,
                    Some("Message::from_command() failed to create a message from given command.".to_string())));
            }
        }
    }

    /// Creates a [Message] from [Command::Send]. Used inside [Message::from_command].
    fn from_send(message_kind: MessageKind,
                 author: User, recipients: Vec<String>,
                 content: Vec<u8>, file_name: Option<String>) -> Result<Self, NetCommsError> {

        let mut message = Self::new()?;

        let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
        message.set_metadata(metadata);

        message.set_content(content);

        let end_data = Packet::new(PacketKind::End);
        message.set_end_data(end_data);

        Ok(message)    
    }

    /// Creates a [Message] from [Command::Register]. Used inside [Message::from_command].
    fn from_register(user_unchecked: UserUnchecked, author: User) -> Result<Self, NetCommsError> {

        let mut message = Self::new()?;

        let request = Request::Register(user_unchecked);
        let content = request.to_ron()?.to_buff()?;

        // Recipient of Request will always be a server.
        let message_kind = MessageKind::Request;
        let recipients = vec![SERVER_USERNAME.to_string().clone()];
        let file_name = None;

        let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
        message.set_metadata(metadata);

        message.set_content(content);

        let end_data = Packet::new(PacketKind::End);
        message.set_end_data(end_data);

        Ok(message)  
    }

    /// Creates a [Message] from [Command::Login]. Used inside [Message::from_command].
    fn from_login(user_unchecked: UserUnchecked, author: User) -> Result<Self, NetCommsError> {

        let mut message = Self::new()?;

        let request = Request::Login(user_unchecked);
        let content = request.to_ron()?.to_buff()?;

        // Recipient of Request will always be a server.
        let message_kind = MessageKind::Request;
        let recipients = vec![SERVER_USERNAME.to_string().clone()];
        let file_name = None;

        let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
        message.set_metadata(metadata);

        message.set_content(content);

        let end_data = Packet::new(PacketKind::End);
        message.set_end_data(end_data);

        Ok(message)  
    }

    /// Creates a [Message] from [Request].
    ///
    /// This method is used for [requests](Request) that are not coming from command, so do not have its own method
    /// like [Request::Register] or [Request::Login] for example.
    ///
    /// # Fields
    ///
    /// * `request` -- [Request] from which this method is supposed to create a [Message].
    /// * `author` -- [User] of client that created this [Request].
    ///
    /// # Examples
    ///
    /// ```
    /// // Users are not usually and should not be created like that,
    /// // here it is used only for purpose of this example.
    /// let user = User::new(1, "some_username".to_string(), "some_password".to_string());
    /// let request = Request::GetWaitingMessages;
    /// let message = Message::from_request(request, user.clone()).unwrap();
    /// ```
    /// # Errors
    ///
    /// * Can return [NetCommsError] when serializing requests or other structs, or when using [to_buff](crate::buffer::ToBuffer::to_buff)
    /// on some types.
    pub fn from_request(request: Request, author: User) -> Result<Self, NetCommsError> {

        let mut message = Message::new()?;
        let content = request.to_ron()?.to_buff()?;

        // Recipient of Request will always be a server.
        let message_kind = MessageKind::Request;
        let recipients = vec![SERVER_USERNAME.to_string().clone()];
        let file_name = None;

        let metadata = MetaData::new(&content, message_kind, author, SERVER_ID, recipients, file_name)?;
        message.set_metadata(metadata);

        message.set_content(content);

        let end_data = Packet::new(PacketKind::End);
        message.set_end_data(end_data);

        Ok(message)  
    }

    /// This takes an ownership of self and sends a Message through given [stream](TcpStream).
    ///
    /// This method is used for all [message kinds](MessageKind) except [MessageKind::File]
    /// as file has its own method to sent it.
    ///
    /// # Examples
    ///
    /// ```
    /// let socket = "127.0.0.1:6969";
    /// let mut stream = TcpStream::connect(socket).unwrap();
    ///
    /// let message = Message::new().unwrap();
    /// message.send(&mut stream).unwrap();
    /// ```
    ///
    /// # Errors
    /// 
    /// * Can return [NetCommsError] with kind [NetCommsErrorKind::WritingToStreamFailed] when there is an error
    /// while writing to a stream.
    pub fn send(self, stream: &mut TcpStream) -> Result<(), NetCommsError> {

        Self::send_metadata(self.metadata(), stream)?;

        Self::send_content(self.content, stream)?;
        
        // Write an end_data packet to stream.
        Self::send_end_data(self.end_data, stream)?;

        Ok(())
    }

    /// This takes an ownership of self and sends a Message through given [stream](TcpStream).
    ///
    /// Unlike [Message::send] this is supposed to send only files, as inside a [Message] with
    /// [MessageKind::File] is not the file stored directly. There is only its location on the
    /// device so this method can gradually read file to [MAX_PACKET_CONTENT_SIZE] send it
    /// as a [Packet] and then repeat until the end of file is reached. Because of this there
    /// is no risk of overflowing the memory with too big files.
    ///
    /// # Examples
    /// ```
    /// // Users are not usually and should not be created like that,
    /// // here it is used only for purpose of this example.
    /// let author = User::new(1, "some_username".to_string(), "some_password".to_string());
    /// let socket = "127.0.0.1:6969";
    /// let mut stream = TcpStream::connect(socket).unwrap();
    ///
    /// let mut message = Message::new().unwrap();
    /// 
    /// let path_to_file = "some_directory//some_existing_file.rs"
    /// let metadata = MetaData::new(Vec::new(), 
    ///                              MessageKind::File, author,
    ///                              42, vec!["one_who_knows".to_string()],
    ///                              Some(path_to_file.to_string()));
    /// message.set_metadata(metadata);
    /// message.send_file(&mut stream).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// * This method can return an error when opening and reading from file.
    /// * Also can return [NetCommsError] with kind [NetCommsErrorKind::WritingToStreamFailed] when there is an error
    /// while writing to a stream.
    /// * Also can returns an [NetCommsErrorKind::IncompleteMetaData].
    /// * Can return [NetCommsError] when serializing requests or other structs, or when using [to_buff](crate::buffer::ToBuffer::to_buff)
    /// on some types.
    pub fn send_file(self, stream: &mut TcpStream) -> Result<(), NetCommsError> {

        if let Some(file_name) = self.metadata.file_name() {
            match File::open(&file_name) {
                Ok(file) => {

                    // Internal methods used to send file.
                    
                    let metadata = Self::change_metadata_according_to_file(&file, self.metadata())?;
                    Self::send_metadata(metadata, stream)?;

                    Self::send_file_packets(&file, stream)?;

                    Self::send_end_data(self.end_data, stream)?;     
                    
                    Ok(())
                }
                Err(e) => return Err(NetCommsError::new(
                    NetCommsErrorKind::OpeningFileFailed,
                    Some(format!("Opening a file {} failed. ({})", file_name, e)))),
            }
        } else {
            Err(NetCommsError::new(
                NetCommsErrorKind::IncompleteMetaData,
                Some("File can not be sent, missing file name".to_string())))
        }            
    }

    /// Sends `metadata` through [TcpStream]. Used inside [Message::send] and [Message::send_file].
    fn send_metadata(metadata: MetaData, stream: &mut TcpStream) -> Result<(), NetCommsError> {

         // Create multiple metadata packets if necessary and write them to stream.
         let metadata_buff = metadata.to_buff()?;
         let metadata_buff_split = Self::split_to_max_packet_size(metadata_buff);
         
         let mut id = 0;    // id is used to know when end of metadata is reached, so MetaDataEnd can be send.
         let n_of_metadata_packets = metadata_buff_split.len();
         for buff in metadata_buff_split {
             id += 1;
             let packet: Packet;
             if id == n_of_metadata_packets {
                 packet = Packet::new(PacketKind::new_metadata_end(buff));
             } else {
                 packet = Packet::new(PacketKind::new_metadata(buff));
             }
             
             if let Err(e) = stream.write(&packet.to_buff()?) {
                 return Err(NetCommsError::new(
                     NetCommsErrorKind::WritingToStreamFailed,
                     Some(format!("Failed to write metadata packet to stream. ({})", e))));
             }
         }
         Ok(())
    }

    /// Sends `file` through [TcpStream]. Used inside [Message::send_file].
    ///
    /// This also directly reads the file, it is not passed to it through arguments.
    /// Always sends part of file with size equal to [MAX_PACKET_CONTENT_SIZE] to prevent 
    /// overflowing the memory.
    fn send_file_packets(file: &File, stream: &mut TcpStream) -> Result<(), NetCommsError> {

        // Get information about to how many packet the file will be split.
        let n_of_content_packets = Self::n_of_content_packets(file.metadata().unwrap().len() as usize);
        
        let mut reader = BufReader::new(file);
        // Starts at 1 and ends inclusively at n_of_content_packets so the whole file is read.
        for part in 1..=n_of_content_packets {
            let packet: Packet;
            {
                let mut buff: Vec<u8>;   
                if part == n_of_content_packets {
                    buff = Vec::new();
                    if let Err(e) = reader.read_to_end(&mut buff) {
                        return Err(NetCommsError::new(
                            NetCommsErrorKind::ReadingFromFileFailed,
                            Some(format!("Failed to read last content packet from file. ({})", e))));
                    }
                } else {
                    // Create a buffer with exact buffer size.
                    buff = vec![0_u8; MAX_PACKET_CONTENT_SIZE];
                    if let Err(e) = reader.read_exact(&mut buff) {  // This read_exact instead of read is really important.
                        return Err(NetCommsError::new(
                            NetCommsErrorKind::ReadingFromFileFailed,
                            Some(format!("Failed to read content packet from file. ({})", e))));
                    } 
                }    

                packet = Packet::new(PacketKind::new_content(buff));
            }

            if let Err(e) = stream.write(&packet.to_buff()?) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::WritingToStreamFailed,
                    Some(format!("Failed to write content packet (file) to stream. ({})", e))));
            }
        }

        Ok(())        
    }
    
    /// Get the number of content packets needed to send whole file.
    fn n_of_content_packets(file_length: usize) -> usize {
        let n_of_content_packets: usize;

        if file_length % MAX_PACKET_SIZE != 0 {
            // Add one more packet for one not full content packet at the end.
            n_of_content_packets = (file_length as usize / (MAX_PACKET_CONTENT_SIZE)) + 1;
        } else {
            n_of_content_packets = file_length as usize / (MAX_PACKET_CONTENT_SIZE);
        }

        n_of_content_packets
    }

    /// Change `file_name` in `metadata` from whole [Path] to file name with its extension only.
    fn change_metadata_according_to_file(file: &File, metadata: MetaData) -> Result<MetaData, NetCommsError> {

        let file_length = file.metadata().unwrap().len(); // Could not find in what case this returns an error, will check later if necessary.

        let n_of_content_packets = Self::n_of_content_packets(file_length as usize);

        // This operation is not computationally heavy as there is only transfer of ownership, no cloning.
        let metadata_buff = metadata.to_buff()?;
        let n_of_metadata_packets = Self::number_of_packets(&metadata_buff);
                    
        let n_of_packets = n_of_metadata_packets + n_of_content_packets + 1;

        let mut metadata = MetaData::from_buff(metadata_buff)?;
        metadata.set_message_length(n_of_packets);

        // Here is no need for complex error handling as this method is used after the file existence check.
        let path = metadata.file_name().unwrap();
        let path = Path::new(&path);
        let file_name = path.file_name().map(|name| name.to_string_lossy().into_owned());

        metadata.set_file_name(file_name);
        Ok(metadata)
    }

    /// Sends `content` through [TcpStream]. Used inside [Message::send].
    fn send_content(content: Vec<Packet>, stream: &mut TcpStream) -> Result<(), NetCommsError> {

        // Write all content packets to stream.
        for packet in content.into_iter() {
            if let Err(e) = stream.write_all(&packet.to_buff()?) {
                return Err(NetCommsError::new(
                    NetCommsErrorKind::WritingToStreamFailed,
                    Some(format!("Failed to write a buffer to stream. ({})", e))));
            }
        }

        Ok(())
    }

    /// Sends `end_data` through [TcpStream]. Used inside [Message::send] and [Message::send_file].
    fn send_end_data(end_data: Packet, stream: &mut TcpStream) -> Result<(), NetCommsError> {

        if let Err(e) = stream.write(&end_data.to_buff()?) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::WritingToStreamFailed,
                Some(format!("Failed to write end data packet to stream. ({})", e))));
        }  

        Ok(())                
    }

    /// Receives a Message from given stream.
    ///
    /// # Arguments
    /// `stream` -- [TcpStream] used to receive the [Message].
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// // Users are not usually and should not be created like that,
    /// // here it is used only for purpose of this example.
    /// let author = User::new(1, "some_username".to_string(), "some_password".to_string());
    /// let socket = "127.0.0.1:6969";
    /// let mut stream = TcpStream::connect(socket).unwrap();
    ///
    /// let message = Message::receive(&mut stream).unwrap();
    /// ```
    /// Work with `message` based on its kind.
    /// ```
    /// # let author = User::new(1, "some_username".to_string(), "some_password".to_string());
    /// # let socket = "127.0.0.1:6969";
    /// # let mut stream = TcpStream::connect(socket).unwrap();
    /// # let message = Message::receive(&mut stream).unwrap();    
    /// match message.kind() {
    ///     _ => {
    ///         // Do some stuff based on message kind.  
    ///     }   
    /// }
    /// ```
    /// 
    /// # Errors
    /// * Will return an [NetCommsError] with kind [NetCommsErrorKind::InvalidPacketKind] if it fails to read correct packet kind, or unexpected
    /// [PacketKind] is received.
    /// * Will return an [NetCommsError] with kind [NetCommsErrorKind::InvalidBufferSize] if it fails to create [Packet] or [MetaData] from buffer.
    /// * Can also return other errors when receiving a file.
    // Some refactoring and upgrades need to be done.
    // First of all saving files to correct location. Not it is just in same folder as is binary.
    // Then solve situation when the file with same name as already existing file is received.
    pub fn receive(stream: &mut TcpStream) -> Result<Self, NetCommsError> {

        // Create new empty Message.
        let mut message = Message::new()?;
        
        // Receive MetaData.
        let metadata = Self::receive_metadata(stream)?;        
        message.set_metadata(metadata);

        // Receive content. This method also receives end_data to know when stop receiving.
        Self::receive_content(&mut message, stream)?;  
        
        Ok(message)
    }

    /// Receives a [MetaData] for a [Message]. Used inside [Message::receive].
    fn receive_metadata(stream: &mut TcpStream) -> Result<MetaData, NetCommsError> {

        let mut metadata_buff = Vec::new(); 

        // Loop to read all metadata packets.
        loop {   
            let packet = Self::receive_packet(stream)?;         
            match packet.kind() {
                PacketKind::MetaData(..) => {
                    metadata_buff.extend(packet.kind_owned().content()?);
                }
                PacketKind::MetaDataEnd(..) => {
                    metadata_buff.extend(packet.kind_owned().content()?);
                    break;
                },
                _ => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some(format!("Unexpected PacketKind, expected MetaData or MetaDataEnd, arrived:\n {:?}", packet.kind_owned()))));
                }               
            }
        }
        Ok(MetaData::from_buff(metadata_buff)?)
    }

    /// Receives a `content` writes it into `message`. Used inside [Message::receive].
    fn receive_content(message: &mut Message, stream: &mut TcpStream) -> Result<(), NetCommsError> {

        // Loop to read all content packets.
        loop {         
            let packet = Self::receive_packet(stream)?;

            // Get a packet kind and modify a message based on that. 
            match packet.kind() {
                PacketKind::Content(..) => {
                    match message.metadata().message_kind() {
                        // An error in case MessageKind is unknown.
                        MessageKind::Unknown => {
                            return Err(NetCommsError::new(
                                NetCommsErrorKind::UnknownMessageKind,
                                None));
                        },
                        // File needs to be threated differently, other variants are pushed into content vec.
                        MessageKind::File => {
                            // Cases where the file with the same name that already exists is send, should be solved,
                            // as for now it just appends incoming content to the current.
                            let mut file = OpenOptions::new()
                                                            .create(true)
                                                            .append(true)
                                                            .write(true)
                                                            .open(message.metadata().file_name().unwrap()) //Unwrap can be used.
                                                            .unwrap();

                            if let Err(e) = file.write(&packet.kind_owned().content()?) {
                                return Err(NetCommsError::new(
                                    NetCommsErrorKind::WritingToFileFailed,
                                    Some(format!("Could not write to file. ({})", e))));
                            }                    
                        }
                        _ => {
                            message.push_content(packet);
                        }
                    }
                },
                PacketKind::End => {
                    message.set_end_data(packet);
                    break;
                },
                _ => {
                    return Err(NetCommsError::new(
                        NetCommsErrorKind::InvalidPacketKind, 
                        Some(format!("Unexpected PacketKind, expected Content or End, arrived:\n {:?}", packet.kind_owned()))));
                }
            } 
        }
        Ok(())
    }

    /// Receives a single [Packet] from [TcpStream]. Used inside [Message::receive_metadata] and [Message::receive_content].
    fn receive_packet(stream: &mut TcpStream) -> Result<Packet, NetCommsError> {

        // Read the size of packet.
        let mut size_buff = vec![0_u8; 8];
        if let Err(e) = stream.read_exact(&mut size_buff) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::ReadingFromStreamFailed, 
                Some(format!("Failed to read the size of packet. \n({})", e))));
        }
        let size = usize::from_buff(size_buff.clone())?;

        // Read rest of packet.
        let mut buff = vec![0_u8; size - 8];    // - 8 for size of packet encoded as bytes which already exist.
        if let Err(e) = stream.read_exact(&mut buff) {
            return Err(NetCommsError::new(
                NetCommsErrorKind::ReadingFromStreamFailed, 
                Some(format!("Failed to read the contents of packet. \n({})", e))));
        }

        // Connect whole buffer and change name, so it makes more sense.
        size_buff.extend(buff);
        let buff = size_buff;
        
        // Create and return a packet from buffer.
        Ok(Packet::from_buff(buff)?)
    }

    /// Sets a `metadata` of [Message]. Also sets message kind.
    ///
    /// Takes an ownership of `metadata` given in argument.
    pub fn set_metadata(&mut self, metadata: MetaData) {

        self.kind = metadata.message_kind();
        self.metadata = metadata;
    }

    /// Adds a new [Packet] to [Message] content.
    ///
    /// Takes an ownership of `packet` given in argument. 
    pub fn push_content(&mut self, packet: Packet) {
        self.content.push(packet);
    }

    /// Sets `content` of [Message].
    ///
    /// This takes an ownership of `content` in argument.
    ///
    /// This is preferred way of setting a content for [Message].
    pub fn set_content(&mut self, content: Vec<u8>) {
        self.content = Self::split_to_max_packet_size(content)
                            .into_iter()
                            .map(|buffer| Packet::new(PacketKind::new_content(buffer)))
                            .collect();
    }

    /// Sets an `end_data` of [Message].
    ///
    /// Takes an ownership of `end_data` given in argument.
    pub fn set_end_data(&mut self, end_data: Packet) {
        self.end_data = end_data;
    }

    /// Returns a [MessageKind].
    ///
    /// It is a low const operation, so `kind` is cloned.
    pub fn kind(&self) -> MessageKind {
        self.kind.clone()
    }

    // Returns [Message] `metadata`.
    ///
    /// It is usually a low const operation, so `kind` is cloned, but owned version exist as well.
    pub fn metadata(&self) -> MetaData {
        self.metadata.clone()
    }

    /// Returns [Message] `metadata`.
    ///
    /// This takes an ownership of `self`. and returns owned [MetaData].
    pub fn metadata_owned(self) -> MetaData {
        self.metadata
    }

    /// Returns [Message] `content` in form of one [Vec] of [u8] so [from_buff](crate::buffer::FromBuffer::from_buff)
    /// can be used on it.
    ///
    /// This takes an ownership of `self`.
    pub fn content(self) ->  Vec<u8> {
        let mut content: Vec<u8> = Vec::new();
        for data in self.content.into_iter() {
            if let PacketKind::Content(_, data) = data.kind_owned() { 
                content.extend(data);
            }
        }
        content
    }

    /// Returns [Message] `content` in form of one [Vec] of [Packet].
    ///
    /// This takes an ownership of `self`.
    pub fn content_split(self) -> Vec<Packet> {
        self.content
    }

    /// Returns a number of packets that will need to be created from given buffer.
    ///
    /// It uses [MAX_PACKET_CONTENT_SIZE] to determine the number.
    pub fn number_of_packets(content: &Vec<u8>) -> usize {

        let byte_length = content.len();

        // Get number of packets by dividing by MAX_PACKET_CONTENT_SIZE.
        let mut number_of_packets = byte_length / MAX_PACKET_CONTENT_SIZE;  
        // Add one packet if there is any remainder after the division.
        if byte_length % MAX_PACKET_CONTENT_SIZE != 0 {
            number_of_packets += 1;
        }
        number_of_packets
    }

    /// Splits the given buffer to vector of buffers with size of [MAX_PACKET_CONTENT_SIZE], so
    /// it can be used to create [packets](Packet).
    pub fn split_to_max_packet_size(buffer: Vec<u8>) -> Vec<Vec<u8>> {

        // This splits given buffer to multiple owned chunks with chunks method from itertools crate,
        // then it will split every chunk to iterator as well which are then collected to vectors of bytes,
        // that are collected to single vector. 
        // This is not my work: https://stackoverflow.com/a/67009164. 
        let vectored_content: Vec<Vec<u8>> = buffer.into_iter()
                                                    .chunks(MAX_PACKET_CONTENT_SIZE)
                                                    .into_iter()
                                                    .map(|chunk| chunk.collect())
                                                    .collect();

        vectored_content
    }
}
