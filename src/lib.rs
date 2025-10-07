use std::io::{
    Write,
    Read
};
#[cfg(feature = "smol")]
use smol::io::{ AsyncWrite, AsyncRead };


pub mod varint;


pub trait Protocol { }


pub trait Encode<P : Protocol> {
    type Error;
    fn encode<W : Write>(&self, writer : W) -> Result<(), Self::Error>;
}

#[cfg(feature = "smol")]
pub trait AsyncEncode<P : Protocol> {
    type Error;
    fn async_encode<W : AsyncWrite + Unpin>(&self, writer : W) -> impl Future<Output = Result<(), Self::Error>>;
}


pub trait Decode<P : Protocol> : Sized {
    type Error;
    fn decode<R : Read>(reader : R) -> Result<Self, Self::Error>;
}

#[cfg(feature = "smol")]
pub trait AsyncDecode<P : Protocol> : Sized {
    type Error;
    fn async_decode<R : AsyncRead + Unpin>(reader : R) -> impl Future<Output = Result<Self, Self::Error>>;
}
