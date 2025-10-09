use crate::{
    Protocol,
    NetEncode,
    NetDecode
};
use core::marker::PhantomData;
use std::{
    borrow::Cow,
    io,
    string::FromUtf8Error
};
use smol::io::{
    AsyncWrite, AsyncWriteExt,
    AsyncRead, AsyncReadExt
};


#[non_exhaustive]
pub struct Utf8<Len, LenProtocol : Protocol> {
    _marker : PhantomData<(Len, LenProtocol,)>
}
impl<Len, LenProtocol : Protocol> Protocol for Utf8<Len, LenProtocol> { }


impl<Len, LenProtocol : Protocol> NetDecode<Utf8<Len, LenProtocol>> for String
where Len : NetDecode<LenProtocol> + Into<usize>
{
    type Error = Utf8DecodeError<Len, LenProtocol>;
    async fn decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self, Self::Error> {
        let     len = Len::decode(&mut reader).await.map_err(Utf8DecodeError::Len)?.into();
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf).await.map_err(Utf8DecodeError::Io)?;
        String::from_utf8(buf).map_err(Utf8DecodeError::Utf8)
    }
}
impl<Len, LenProtocol : Protocol> NetDecode<Utf8<Len, LenProtocol>> for Cow<'_, str>
where Len : NetDecode<LenProtocol> + Into<usize>
{
    type Error = Utf8DecodeError<Len, LenProtocol>;
    async fn decode<R : AsyncRead + Unpin>(reader : R) -> Result<Self, Self::Error> {
        Ok(Cow::Owned(String::decode(reader).await?))
    }
}

pub enum Utf8DecodeError<Len, LenProtocol : Protocol>
where Len : NetDecode<LenProtocol>
{
    Len(Len::Error),
    Io(io::Error),
    Utf8(FromUtf8Error)
}


impl<Len, LenProtocol : Protocol> NetEncode<Utf8<Len, LenProtocol>> for str
where Len : NetEncode<LenProtocol> + From<usize>
{
    type Error = Utf8EncodeError<Len, LenProtocol>;
    async fn encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result<(), Self::Error> {
        let b = self.as_bytes();
        Len::from(b.len()).encode(&mut writer).await.map_err(Utf8EncodeError::Len)?;
        writer.write_all(b).await.map_err(Utf8EncodeError::Io)?;
        Ok(())
    }
}
impl<Len, LenProtocol : Protocol> NetEncode<Utf8<Len, LenProtocol>> for Cow<'_, str>
where Len : NetEncode<LenProtocol> + From<usize>
{
    type Error = Utf8EncodeError<Len, LenProtocol>;
    async fn encode<W : AsyncWrite + Unpin>(&self, writer : W) -> Result<(), Self::Error> {
        <&str>::encode(&&**self, writer).await
    }
}
impl<Len, LenProtocol : Protocol> NetEncode<Utf8<Len, LenProtocol>> for String
where Len : NetEncode<LenProtocol> + From<usize>
{
    type Error = Utf8EncodeError<Len, LenProtocol>;
    async fn encode<W : AsyncWrite + Unpin>(&self, writer : W) -> Result<(), Self::Error> {
        <&str>::encode(&&**self, writer).await
    }
}

pub enum Utf8EncodeError<Len, LenProtocol : Protocol>
where Len : NetEncode<LenProtocol>
{
    Len(Len::Error),
    Io(io::Error)
}
