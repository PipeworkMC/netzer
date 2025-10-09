use smol::io::{ AsyncWrite, AsyncRead };


pub trait EncodeWith<T, W : AsyncWrite + Unpin> {
    type Error;
    fn encode(&mut self, v : &T, writer : W) -> impl Future<Output = Result<(), Self::Error>>;
}

impl<Error, T, W : AsyncWrite + Unpin, F : AsyncFnMut(&T, W) -> Result<(), Error>> EncodeWith<T, W> for F {
    type Error = Error;
    async fn encode(&mut self, v : &T, writer : W) -> Result<(), Self::Error> {
        (self)(v, writer).await
    }
}


pub trait DecodeWith<T, R : AsyncRead + Unpin> {
    type Error;
    fn decode(&mut self, reader : R) -> impl Future<Output = Result<T, Self::Error>>;
}

impl<Error, T, R : AsyncRead + Unpin, F : AsyncFnMut(R) -> Result<T, Error>> DecodeWith<T, R> for F {
    type Error = Error;
    async fn decode(&mut self, reader : R) -> Result<T, Self::Error> {
        (self)(reader).await
    }
}
