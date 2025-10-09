use std::io::{ Write, Read };
#[cfg(feature = "smol")]
use smol::io::{ AsyncWrite, AsyncRead };


pub trait EncodeWith<T, W : Write> {
    type Error;
    fn encode(&mut self, v : &T, writer : W) -> Result<(), Self::Error>;
}

impl<Error, T, W : Write, F : FnMut(&T, W) -> Result<(), Error>> EncodeWith<T, W> for F {
    type Error = Error;
    fn encode(&mut self, v : &T, writer : W) -> Result<(), Self::Error> {
        (self)(v, writer)
    }
}

#[cfg(feature = "smol")]
pub trait AsyncEncodeWith<T, W : AsyncWrite + Unpin> {
    type Error;
    fn async_encode(&mut self, v : &T, writer : W) -> impl Future<Output = Result<(), Self::Error>>;
}

#[cfg(feature = "smol")]
impl<Error, T, W : AsyncWrite + Unpin, F : AsyncFnMut(&T, W) -> Result<(), Error>> AsyncEncodeWith<T, W> for F {
    type Error = Error;
    fn async_encode(&mut self, v : &T, writer : W) -> impl Future<Output = Result<(), Self::Error>> {
        (self)(v, writer)
    }
}


pub trait DecodeWith<T, R : Read> {
    type Error;
    fn decode(&mut self, reader : R) -> Result<T, Self::Error>;
}

impl<Error, T, R : Read, F : FnMut(R) -> Result<T, Error>> DecodeWith<T, R> for F {
    type Error = Error;
    fn decode(&mut self, reader : R) -> Result<T, Self::Error> {
        (self)(reader)
    }
}

#[cfg(feature = "smol")]
pub trait AsyncDecodeWith<T, R : AsyncRead + Unpin> {
    type Error;
    fn async_decode(&mut self, reader : R) -> impl Future<Output = Result<T, Self::Error>>;
}

#[cfg(feature = "smol")]
impl<Error, T, R : AsyncRead + Unpin, F : AsyncFnMut(R) -> Result<T, Error>> AsyncDecodeWith<T, R> for F {
    type Error = Error;
    fn async_decode(&mut self, reader : R) -> impl Future<Output = Result<T, Self::Error>> {
        (self)(reader)
    }
}
