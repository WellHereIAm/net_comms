// use crate::buffer::{IntoBuffer, FromBuffer};
use crate::ron::{FromRon, IntoRon};
use crate::bytes::{FromBytes, IntoBytes};

pub trait MessageKindType<'a>:
    Default + Clone  + FromRon<'a> + IntoRon + FromBytes + IntoBytes {}