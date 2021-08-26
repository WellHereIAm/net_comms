use crate::buffer::{IntoBuffer, FromBuffer};
use crate::ron::{FromRon, IntoRon};

pub trait MessageKindType<'a>:
    Default + Clone  + FromRon<'a> + IntoRon + FromBuffer + IntoBuffer {}