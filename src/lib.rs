use core::fmt::Arguments;
use std::{
    borrow::Cow,
    io::{ self, Write, Read }
};
use smol::io::{
    AsyncWriteExt,
    AssertAsync
};


pub mod prelude {
    pub use crate::{
        AsyncWrite as _,
        AsyncRead as _,
        numeric::{ BigEndian, LittleEndian },
        string::Utf8,
        varint::{ VarInt, Leb128 },
        EncodeWith,
        DecodeWith,
        NetEncode,
        SyncNetEncode as _,
        NetDecode,
        SyncNetDecode as _
    };
}


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


#[cfg(feature = "derive")]
pub use netzer_derive::NetEncode;

pub trait NetEncode<P : Protocol> {
    type Error;
    fn encode<W : AsyncWrite>(&self, writer : W) -> impl Future<Output = Result<(), Self::Error>>;
}
impl<P : Protocol, T : NetEncode<P> + ?Sized> NetEncode<P> for &T {
    type Error = T::Error;
    #[inline]
    fn encode<W : AsyncWrite>(&self, writer : W) -> impl Future<Output = Result<(), Self::Error>> {
        T::encode(self, writer)
    }
}
impl<P : Protocol, T : NetEncode<P> + ?Sized> NetEncode<P> for &mut T {
    type Error = T::Error;
    #[inline]
    fn encode<W : AsyncWrite>(&self, writer : W) -> impl Future<Output = Result<(), Self::Error>> {
        T::encode(self, writer)
    }
}

pub trait SyncNetEncode<P : Protocol> : NetEncode<P> {
    fn sync_encode<W : Write>(&self, writer : W) -> Result<(), Self::Error>;
}
impl<P : Protocol, T : NetEncode<P>> SyncNetEncode<P> for T {
    fn sync_encode<W : Write>(&self, writer : W) -> Result<(), Self::Error> {
        smol::block_on(self.encode(AssertAsync::new(writer)))
    }
}


#[cfg(feature = "derive")]
pub use netzer_derive::NetDecode;

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
