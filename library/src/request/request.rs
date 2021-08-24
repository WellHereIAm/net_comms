use serde::{Serialize, Deserialize};

use crate::ron::{ToRon, FromRon};
use crate::user::UserUnchecked;


/// Holds data about requests from client to server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    /// Request to login with [UserUnchecked] inside.
    Login(UserUnchecked),

    /// Request to register with [UserUnchecked] inside.
    Register(UserUnchecked),

    /// Request to get any [messages](crate::message::Message) that were sent to requesting client.
    GetWaitingMessagesAuto,

    /// Used if some method fails to recognize the [Request].
    Unknown,    
}

impl ToRon for Request {}
impl FromRon<'_> for Request {}
