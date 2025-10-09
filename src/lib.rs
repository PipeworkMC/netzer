use core::fmt::Arguments;
use std::{
    borrow::Cow,
    io::{ self, Write, Read }
};
use smol::io::{
    AsyncWriteExt,
    AssertAsync
};


pub trait AsyncWrite : smol::io::AsyncWrite + Unpin {
    fn write_fmt(&mut self, arguments : Arguments<'_>) -> impl Future<Output = Result<(), io::Error>>;
}
impl<T : smol::io::AsyncWrite + Unpin> AsyncWrite for T {
    async fn write_fmt(&mut self, arguments : Arguments<'_>) -> Result<(), io::Error> {
        let s = arguments.as_str().map_or_else(|| Cow::Owned(arguments.to_string()), Cow::Borrowed);
        self.write_all(s.as_bytes()).await
    }
}

pub trait AsyncRead : smol::io::AsyncRead + Unpin { }
impl<T : smol::io::AsyncRead + Unpin> AsyncRead for T { }


pub mod numeric;
pub mod string;
pub mod varint;

mod with;
pub use with::*;


pub trait Protocol { }


pub use netzer_derive::NetEncode;

pub trait NetEncode<P : Protocol> {
    type Error;
    fn encode<W : AsyncWrite>(&self, writer : W) -> impl Future<Output = Result<(), Self::Error>>;
}

pub trait SyncNetEncode<P : Protocol> : NetEncode<P> {
    fn sync_encode<W : Write>(&self, writer : W) -> Result<(), Self::Error>;
}
impl<P : Protocol, T : NetEncode<P>> SyncNetEncode<P> for T {
    fn sync_encode<W : Write>(&self, writer : W) -> Result<(), Self::Error> {
        smol::block_on(self.encode(AssertAsync::new(writer)))
    }
}


// TODO: pub use netzer_derive::NetDecode;

pub trait NetDecode<P : Protocol> : Sized {
    type Error;
    fn decode<R : AsyncRead>(reader : R) -> impl Future<Output = Result<Self, Self::Error>>;
}

pub trait SyncNetDecode<P : Protocol> : NetDecode<P> + Sized {
    fn sync_decode<R : Read>(reader : R) -> Result<Self, Self::Error>;
}
impl<P : Protocol, T : NetDecode<P>> SyncNetDecode<P> for T {
    fn sync_decode<R : Read>(reader : R) -> Result<Self, Self::Error> {
        smol::block_on(Self::decode(AssertAsync::new(reader)))
    }
}
