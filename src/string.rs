use crate::{
    Protocol,
    NetEncode,
    NetDecode,
    Result, Error
};
use core::marker::PhantomData;
use std::borrow::Cow;
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
where
    Len   : NetDecode<LenProtocol> + TryInto<usize>,
    Error : From<<Len as TryInto<usize>>::Error>
{
    async fn decode<R : AsyncRead + Unpin>(mut reader : R) -> Result<Self> {
        let     len = Len::decode(&mut reader).await?
            .try_into()?;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf).await?;
        Ok(String::from_utf8(buf)?)
    }
}
impl<Len, LenProtocol : Protocol> NetDecode<Utf8<Len, LenProtocol>> for Cow<'_, str>
where
    Len    : NetDecode<LenProtocol> + TryInto<usize>,
    Error : From<<Len as TryInto<usize>>::Error>
{
    async fn decode<R : AsyncRead + Unpin>(reader : R) -> Result<Self> {
        Ok(Cow::Owned(<String as NetDecode<Utf8<Len, LenProtocol>>>::decode(reader).await?))
    }
}


impl<Len, LenProtocol : Protocol> NetEncode<Utf8<Len, LenProtocol>> for str
where
    Len   : NetEncode<LenProtocol> + TryFrom<usize>,
    Error : From<<Len as TryFrom<usize>>::Error>
{
    async fn encode<W : AsyncWrite + Unpin>(&self, mut writer : W) -> Result {
        let b = self.as_bytes();
        Len::try_from(b.len())?
            .encode(&mut writer).await?;
        writer.write_all(b).await?;
        Ok(())
    }
}
impl<Len, LenProtocol : Protocol> NetEncode<Utf8<Len, LenProtocol>> for Cow<'_, str>
where
    Len   : NetEncode<LenProtocol> + TryFrom<usize>,
    Error : From<<Len as TryFrom<usize>>::Error>
{
    async fn encode<W : AsyncWrite + Unpin>(&self, writer : W) -> Result {
        <&str as NetEncode<Utf8<Len, LenProtocol>>>::encode(&&**self, writer).await
    }
}
impl<Len, LenProtocol : Protocol> NetEncode<Utf8<Len, LenProtocol>> for String
where
    Len   : NetEncode<LenProtocol> + TryFrom<usize>,
    Error : From<<Len as TryFrom<usize>>::Error>
{
    async fn encode<W : AsyncWrite + Unpin>(&self, writer : W) -> Result {
        <&str as NetEncode<Utf8<Len, LenProtocol>>>::encode(&&**self, writer).await
    }
}
