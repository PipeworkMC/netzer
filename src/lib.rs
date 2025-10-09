use std::io::{
    Write,
    Read
};
pub use smol::io::{ AsyncWrite, AsyncRead };
use smol::io::AssertAsync;


pub mod numeric;
pub mod string;
pub mod varint;

mod with;
pub use with::*;


pub trait Protocol { }



pub use netzer_derive::NetEncode;

pub trait NetEncode<P : Protocol> {
    type Error;
    fn encode<W : AsyncWrite + Unpin>(&self, writer : W) -> impl Future<Output = Result<(), Self::Error>>;
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
    fn decode<R : AsyncRead + Unpin>(reader : R) -> impl Future<Output = Result<Self, Self::Error>>;
}

pub trait SyncNetDecode<P : Protocol> : NetDecode<P> + Sized {
    fn sync_decode<R : Read>(reader : R) -> Result<Self, Self::Error>;
}
impl<P : Protocol, T : NetDecode<P>> SyncNetDecode<P> for T {
    fn sync_decode<R : Read>(reader : R) -> Result<Self, Self::Error> {
        smol::block_on(Self::decode(AssertAsync::new(reader)))
    }
}
