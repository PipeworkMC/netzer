use std::io::{
    Write,
    Read
};
#[cfg(feature = "smol")]
use smol::io::{ AsyncWrite, AsyncRead };


pub mod numeric;
pub mod varint;


pub trait Protocol { }



pub use netzer_derive::NetEncode;

pub trait NetEncode<P : Protocol> {
    type Error;
    fn encode<W : Write>(&self, writer : W) -> Result<(), Self::Error>;
}

#[cfg(feature = "smol")]
pub trait AsyncNetEncode<P : Protocol> {
    type Error;
    fn async_encode<W : AsyncWrite + Unpin>(&self, writer : W) -> impl Future<Output = Result<(), Self::Error>>;
}


// TODO: pub use netzer_derive::Encode;

pub trait NetDecode<P : Protocol> : Sized {
    type Error;
    fn decode<R : Read>(reader : R) -> Result<Self, Self::Error>;
}

#[cfg(feature = "smol")]
pub trait AsyncNetDecode<P : Protocol> : Sized {
    type Error;
    fn async_decode<R : AsyncRead + Unpin>(reader : R) -> impl Future<Output = Result<Self, Self::Error>>;
}
