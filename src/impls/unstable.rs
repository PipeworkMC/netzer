use crate::{
    NetFormat,
    NetEncode,
    NetDecode,
    AsyncWrite,
    AsyncRead
};
use core::{
    error::Error as StdError,
    fmt::{ self, Display, Formatter }
};


impl<F : NetFormat> NetEncode<F> for ! {
    async fn encode<W : AsyncWrite>(&self, _writer : W) -> crate::Result {
        unreachable!()
    }
}
impl<F : NetFormat> NetDecode<F> for ! {
    async fn decode<R : AsyncRead>(_reader : R) -> crate::Result<Self> {
        Err(DecodeNever.into())
    }
}


#[derive(Debug)]
pub struct DecodeNever;
impl StdError for DecodeNever { }
impl Display for DecodeNever {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "can not decode `!`")
    }
}
