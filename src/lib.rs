use core::{
    error::Error as StdError,
    fmt::Arguments
};
use std::{
    borrow::Cow,
    fmt::{ self, Debug, Display, Formatter },
    io::{ Write, Read }
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
    pub use smol::io::{
        AsyncWriteExt as _,
        AsyncReadExt as _
    };
}


pub trait AsyncWrite : smol::io::AsyncWrite + Unpin {
    fn write_fmt(&mut self, arguments : Arguments<'_>) -> impl Future<Output = Result>;
}
impl<T : smol::io::AsyncWrite + Unpin> AsyncWrite for T {
    async fn write_fmt(&mut self, arguments : Arguments<'_>) -> Result {
        let s = arguments.as_str().map_or_else(|| Cow::Owned(arguments.to_string()), Cow::Borrowed);
        self.write_all(s.as_bytes()).await?;
        Ok(())
    }
}

pub trait AsyncRead : smol::io::AsyncRead + Unpin { }
impl<T : smol::io::AsyncRead + Unpin> AsyncRead for T { }


pub mod numeric;
pub mod string;
pub mod varint;

mod with;
pub use with::*;


pub trait NetFormat { }


pub type Error          = Box<dyn StdError>;
pub type Result<T = ()> = core::result::Result<T, Error>;


#[cfg(feature = "derive")]
pub use netzer_derive::NetEncode;

pub trait NetEncode<N : NetFormat> {
    fn encode<W : AsyncWrite>(&self, writer : W) -> impl Future<Output = Result>;
}
impl<N : NetFormat, T : NetEncode<N> + ?Sized> NetEncode<N> for &T {
    #[inline]
    fn encode<W : AsyncWrite>(&self, writer : W) -> impl Future<Output = Result> {
        T::encode(self, writer)
    }
}
impl<N : NetFormat, T : NetEncode<N> + ?Sized> NetEncode<N> for &mut T {
    #[inline]
    fn encode<W : AsyncWrite>(&self, writer : W) -> impl Future<Output = Result> {
        T::encode(self, writer)
    }
}

pub trait SyncNetEncode<N : NetFormat> : NetEncode<N> {
    fn sync_encode<W : Write>(&self, writer : W) -> Result;
}
impl<N : NetFormat, T : NetEncode<N>> SyncNetEncode<N> for T {
    fn sync_encode<W : Write>(&self, writer : W) -> Result {
        smol::block_on(self.encode(AssertAsync::new(writer)))
    }
}


#[cfg(feature = "derive")]
pub use netzer_derive::NetDecode;

pub trait NetDecode<N : NetFormat> : Sized {
    fn decode<R : AsyncRead>(reader : R) -> impl Future<Output = Result<Self>>;
}

pub trait SyncNetDecode<N : NetFormat> : NetDecode<N> + Sized {
    fn sync_decode<R : Read>(reader : R) -> Result<Self>;
}
impl<N : NetFormat, T : NetDecode<N>> SyncNetDecode<N> for T {
    fn sync_decode<R : Read>(reader : R) -> Result<Self> {
        smol::block_on(Self::decode(AssertAsync::new(reader)))
    }
}


#[derive(Debug)]
pub struct BadEnumOrdinal<T>(pub T);
impl<T : Debug + Display> StdError for BadEnumOrdinal<T> { }
impl<T : Display> Display for BadEnumOrdinal<T> {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "bad enum ordinal \"{}\"", self.0.to_string().escape_debug())
    }
}

#[derive(Debug)]
pub struct BadEnumName(pub String);
impl StdError for BadEnumName { }
impl Display for BadEnumName {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        write!(f, "bad enum name {:?}", self.0)
    }
}
